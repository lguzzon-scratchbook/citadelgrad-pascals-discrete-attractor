use std::collections::HashMap;

use async_trait::async_trait;
use attractor_dot::AttributeValue;
use attractor_types::{AttractorError, Context, Outcome, Result, StageStatus};

use crate::graph::{PipelineGraph, PipelineNode};
use crate::handler::NodeHandler;

// ---------------------------------------------------------------------------
// QualityHandler — runs a pipe-separated list of shell quality checks
// ---------------------------------------------------------------------------

pub struct QualityHandler;

#[async_trait]
impl NodeHandler for QualityHandler {
    fn handler_type(&self) -> &str {
        "quality"
    }

    async fn execute(
        &self,
        node: &PipelineNode,
        context: &Context,
        _graph: &PipelineGraph,
    ) -> Result<Outcome> {
        let node_id = &node.id;

        // Check 1: enabled=false in node attrs → skip with success
        if let Some(AttributeValue::Boolean(false)) = node.raw_attrs.get("enabled") {
            return Ok(Outcome {
                status: StageStatus::Success,
                preferred_label: None,
                suggested_next_ids: vec![],
                context_updates: HashMap::new(),
                notes: "Quality checks disabled via node attribute".into(),
                failure_reason: None,
            });
        }

        // Check 2: quality_disabled=true in runtime context → skip with success
        if context
            .get("quality_disabled")
            .await
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            return Ok(Outcome {
                status: StageStatus::Success,
                preferred_label: None,
                suggested_next_ids: vec![],
                context_updates: HashMap::new(),
                notes: "Quality checks disabled via runtime flag".into(),
                failure_reason: None,
            });
        }

        // Get quality_checks attribute (required)
        let checks_str = match node.raw_attrs.get("quality_checks") {
            Some(AttributeValue::String(s)) => s.clone(),
            _ => {
                return Err(AttractorError::HandlerError {
                    handler: "quality".into(),
                    node: node_id.clone(),
                    message: "missing required attribute 'quality_checks'".into(),
                })
            }
        };

        // Split on '|' to get individual commands
        let commands: Vec<&str> = checks_str.split('|').collect();

        // Apply timeout from node config, defaulting to 10 minutes
        let timeout_dur = node
            .timeout
            .unwrap_or(std::time::Duration::from_secs(600));

        let mut results: Vec<serde_json::Value> = Vec::new();
        let mut all_passed = true;
        let mut failure_summaries: Vec<String> = Vec::new();

        for cmd in &commands {
            let cmd = cmd.trim();

            let output =
                match tokio::time::timeout(timeout_dur, tokio::process::Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .output())
                .await
            {
                Ok(Ok(o)) => o,
                Ok(Err(e)) => {
                    return Err(AttractorError::HandlerError {
                        handler: "quality".into(),
                        node: node_id.clone(),
                        message: format!("failed to spawn command '{cmd}': {e}"),
                    })
                }
                Err(_) => {
                    return Err(AttractorError::CommandTimeout {
                        timeout_ms: timeout_dur.as_millis() as u64,
                    })
                }
            };

            let passed = output.status.success();
            let exit_code = output.status.code().unwrap_or(-1);
            let stderr_raw = String::from_utf8_lossy(&output.stderr);
            let stderr = if stderr_raw.len() > 8192 {
                stderr_raw[..8192].to_string()
            } else {
                stderr_raw.trim_end().to_string()
            };

            results.push(serde_json::json!({
                "cmd": cmd,
                "exit_code": exit_code,
                "passed": passed,
                "stderr": stderr,
            }));

            if !passed {
                all_passed = false;
                if !stderr.is_empty() {
                    failure_summaries.push(stderr.clone());
                }
                break; // fail-fast: stop on first failure
            }
        }

        let mut context_updates: HashMap<String, serde_json::Value> = HashMap::new();
        context_updates.insert(
            format!("{node_id}.results"),
            serde_json::Value::Array(results),
        );
        context_updates.insert(
            format!("{node_id}.completed"),
            serde_json::Value::Bool(all_passed),
        );

        if !all_passed {
            let summary = failure_summaries.join("\n");
            context_updates.insert(
                format!("{node_id}.failure_summary"),
                serde_json::Value::String(summary),
            );
            Ok(Outcome {
                status: StageStatus::Fail,
                preferred_label: None,
                suggested_next_ids: vec![],
                context_updates,
                notes: String::new(),
                failure_reason: Some("one or more quality checks failed".into()),
            })
        } else {
            Ok(Outcome {
                status: StageStatus::Success,
                preferred_label: None,
                suggested_next_ids: vec![],
                context_updates,
                notes: String::new(),
                failure_reason: None,
            })
        }
    }
}
