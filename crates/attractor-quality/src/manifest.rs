use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    pub project: ProjectSection,
    pub toolchain: Option<ToolchainSection>,
    pub quality: Option<QualitySection>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectSection {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolchainSection {
    pub language: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QualitySection {
    pub stages: Vec<String>,
    pub max_fix_iterations: Option<u32>,
    #[serde(default)]
    pub hooks: HashMap<String, HookConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HookConfig {
    pub cmd: Option<String>,
    pub cmd_argv: Option<Vec<String>>,
    pub timeout_secs: Option<u64>,
    pub allow_failure: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ResolvedManifest {
    pub manifest: Manifest,
    pub path: PathBuf,
    pub blake3_hash: String,
}
