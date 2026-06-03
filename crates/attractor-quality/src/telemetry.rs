use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StageEvent {
    pub stage: String,
    pub passed: bool,
    pub exit_code: i32,
    pub failure_footprint: Option<String>,
    pub duration_ms: u64,
}

pub fn record(node_id: &str, event: &StageEvent) {
    if event.passed {
        tracing::info!(
            node = %node_id,
            stage = %event.stage,
            duration_ms = event.duration_ms,
            "quality stage passed"
        );
    } else {
        tracing::warn!(
            node = %node_id,
            stage = %event.stage,
            exit_code = event.exit_code,
            footprint = %event.failure_footprint.as_deref().unwrap_or(""),
            duration_ms = event.duration_ms,
            "quality stage failed"
        );
    }
}
