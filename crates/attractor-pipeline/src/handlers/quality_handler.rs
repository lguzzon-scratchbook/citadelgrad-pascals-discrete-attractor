use async_trait::async_trait;
use attractor_types::{Context, Outcome, Result};

use crate::graph::{PipelineGraph, PipelineNode};
use crate::handler::NodeHandler;

// ---------------------------------------------------------------------------
// QualityHandler — runs a pipe-delimited list of quality checks (box shape
// with node_type="quality"). This is a STUB — real implementation pending.
// ---------------------------------------------------------------------------

pub struct QualityHandler;

#[async_trait]
impl NodeHandler for QualityHandler {
    fn handler_type(&self) -> &str {
        "quality"
    }

    async fn execute(
        &self,
        _node: &PipelineNode,
        _context: &Context,
        _graph: &PipelineGraph,
    ) -> Result<Outcome> {
        // STUB: real implementation not yet written.
        // Tests referencing this stub will fail at runtime (RED phase of TDD).
        todo!("QualityHandler not yet implemented")
    }
}
