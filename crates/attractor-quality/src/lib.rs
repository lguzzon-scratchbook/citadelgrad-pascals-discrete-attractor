pub mod manifest;
pub mod resolution;
pub mod detect;
pub mod enrich;
pub mod telemetry;
pub mod trust;

pub use manifest::{HookConfig, Manifest, QualitySection, ResolvedManifest};
pub use resolution::{resolve, ResolutionError};
pub use trust::{add_trust, is_trusted, list_trusted, prompt_and_add, remove_trust, TrustEntry, TrustError};
pub use enrich::enrich;
