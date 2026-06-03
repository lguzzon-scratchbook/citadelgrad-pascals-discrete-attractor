pub mod manifest;
pub mod resolution;
pub mod detect;
pub mod enrich;
pub mod telemetry;
pub mod trust;

pub use manifest::{Manifest, ResolvedManifest};
pub use resolution::{resolve, ResolutionError};
