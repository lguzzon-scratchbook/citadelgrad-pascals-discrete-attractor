use std::path::{Path, PathBuf};

/// A toolchain detected in a project directory.
#[derive(Debug, Clone)]
pub struct DetectedToolchain {
    pub language: String,
    pub config_file: PathBuf,
}

/// Detect toolchains present in `start_dir` by looking for well-known config files.
///
/// Returns all detected toolchains (a repo can have multiple).
pub fn detect(start_dir: &Path) -> Vec<DetectedToolchain> {
    let mut results = Vec::new();

    // Rust
    let cargo = start_dir.join("Cargo.toml");
    if cargo.exists() {
        results.push(DetectedToolchain {
            language: "rust".to_string(),
            config_file: cargo,
        });
    }

    // Python
    let pyproject = start_dir.join("pyproject.toml");
    if pyproject.exists() {
        results.push(DetectedToolchain {
            language: "python".to_string(),
            config_file: pyproject,
        });
    } else {
        let requirements = start_dir.join("requirements.txt");
        if requirements.exists() {
            results.push(DetectedToolchain {
                language: "python".to_string(),
                config_file: requirements,
            });
        }
    }

    // TypeScript / JavaScript
    let package_json = start_dir.join("package.json");
    if package_json.exists() {
        results.push(DetectedToolchain {
            language: "typescript".to_string(),
            config_file: package_json,
        });
    } else {
        let tsconfig = start_dir.join("tsconfig.json");
        if tsconfig.exists() {
            results.push(DetectedToolchain {
                language: "typescript".to_string(),
                config_file: tsconfig,
            });
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn detects_rust() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("Cargo.toml"), "[package]\nname = \"x\"").unwrap();
        let result = detect(tmp.path());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].language, "rust");
    }

    #[test]
    fn detects_python_pyproject() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("pyproject.toml"), "[project]\nname = \"x\"").unwrap();
        let result = detect(tmp.path());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].language, "python");
    }

    #[test]
    fn detects_python_requirements() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("requirements.txt"), "requests\n").unwrap();
        let result = detect(tmp.path());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].language, "python");
    }

    #[test]
    fn detects_typescript_package_json() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("package.json"), "{}").unwrap();
        let result = detect(tmp.path());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].language, "typescript");
    }

    #[test]
    fn detects_multiple_toolchains() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join("Cargo.toml"), "[package]").unwrap();
        fs::write(tmp.path().join("pyproject.toml"), "[project]").unwrap();
        let result = detect(tmp.path());
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn detects_none_in_empty_dir() {
        let tmp = TempDir::new().unwrap();
        let result = detect(tmp.path());
        assert!(result.is_empty());
    }
}
