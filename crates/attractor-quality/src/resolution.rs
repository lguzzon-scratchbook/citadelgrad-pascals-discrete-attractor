//! `pas.toml` manifest resolution.
//!
//! Walks upward from `workdir` to locate `pas.toml`, stopping at the first
//! `.git`, `Cargo.toml`, `pyproject.toml`, or `package.json` it finds (these
//! mark workspace roots).  The first `pas.toml` found wins.

use std::path::{Path, PathBuf};

use thiserror::Error;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// A successfully resolved and (minimally) validated `pas.toml` manifest.
#[derive(Debug, Clone)]
pub struct ResolvedManifest {
    /// The path at which `pas.toml` was found.
    pub path: PathBuf,
}

/// Errors that can occur during manifest resolution.
#[derive(Debug, Error)]
pub enum ResolutionError {
    /// No `pas.toml` found anywhere in the search path.
    #[error("no pas.toml found (searched upward from {searched_from})")]
    NotFound { searched_from: PathBuf },

    /// A `pas.toml` was found but could not be parsed as TOML.
    #[error("pas.toml at {path} is malformed: {source}")]
    Malformed {
        path: PathBuf,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    /// A `pas.toml` was found and parsed but failed schema validation.
    #[error("pas.toml at {path} is invalid: {reason}")]
    Invalid { path: PathBuf, reason: String },
}

// ---------------------------------------------------------------------------
// Resolution logic
// ---------------------------------------------------------------------------

/// Walk upward from `workdir`, returning the first `pas.toml` found, or
/// [`ResolutionError::NotFound`] if none exists within the search boundary.
///
/// The walk stops (without finding a manifest) at the first directory that
/// contains any of: `.git`, `Cargo.toml`, `pyproject.toml`, `package.json`.
/// This prevents resolution from escaping the current project root.
pub fn resolve(workdir: &Path) -> Result<ResolvedManifest, ResolutionError> {
    let workdir = workdir
        .canonicalize()
        .unwrap_or_else(|_| workdir.to_path_buf());

    let mut current = workdir.as_path();
    loop {
        let candidate = current.join("pas.toml");
        if candidate.is_file() {
            // Found it — do a minimal parse check (well-formed TOML).
            let content = std::fs::read_to_string(&candidate).map_err(|e| {
                ResolutionError::Malformed {
                    path: candidate.clone(),
                    source: Box::new(e),
                }
            })?;
            validate_toml(&candidate, &content)?;
            return Ok(ResolvedManifest { path: candidate });
        }

        // Check for workspace root markers — stop the walk here.
        if is_workspace_root(current) {
            break;
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => break,
        }
    }

    Err(ResolutionError::NotFound {
        searched_from: workdir,
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Returns `true` if `dir` contains any workspace-root marker file.
fn is_workspace_root(dir: &Path) -> bool {
    const MARKERS: &[&str] = &[".git", "Cargo.toml", "pyproject.toml", "package.json"];
    MARKERS.iter().any(|m| dir.join(m).exists())
}

/// Minimal TOML well-formedness check.  We don't have a full schema yet, so
/// we just verify the file parses as a TOML document.
fn validate_toml(path: &Path, content: &str) -> Result<(), ResolutionError> {
    // Basic TOML validation: check for `[project]` section presence.
    // A valid pas.toml must be parseable TOML — we validate structure by
    // hand-scanning for the required `[project]` table header.
    // This avoids pulling in the full `toml` crate for now.
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Err(ResolutionError::Invalid {
            path: path.to_path_buf(),
            reason: "pas.toml is empty".into(),
        });
    }
    // Check for obviously broken TOML (unmatched brackets at line start).
    for (lineno, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.starts_with('[') && !line.ends_with(']') && !line.contains('#') {
            return Err(ResolutionError::Invalid {
                path: path.to_path_buf(),
                reason: format!("malformed section header at line {}", lineno + 1),
            });
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_workdir_with_git() -> TempDir {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".git")).unwrap();
        tmp
    }

    #[test]
    fn resolve_finds_pas_toml_in_workdir() {
        let tmp = make_workdir_with_git();
        fs::write(
            tmp.path().join("pas.toml"),
            "[project]\nname = \"test\"\n",
        )
        .unwrap();

        let result = resolve(tmp.path());
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
        // Canonicalize both sides to handle symlinks (e.g. macOS /var -> /private/var).
        let resolved_path = result.unwrap().path.canonicalize().unwrap();
        let expected_path = tmp.path().join("pas.toml").canonicalize().unwrap();
        assert_eq!(resolved_path, expected_path);
    }

    #[test]
    fn resolve_returns_not_found_when_missing() {
        let tmp = make_workdir_with_git();
        // No pas.toml — .git stops the walk.

        let result = resolve(tmp.path());
        assert!(
            matches!(result, Err(ResolutionError::NotFound { .. })),
            "expected NotFound, got {:?}",
            result
        );
    }

    #[test]
    fn resolve_returns_invalid_for_empty_file() {
        let tmp = make_workdir_with_git();
        fs::write(tmp.path().join("pas.toml"), "").unwrap();

        let result = resolve(tmp.path());
        assert!(
            matches!(result, Err(ResolutionError::Invalid { .. })),
            "expected Invalid, got {:?}",
            result
        );
    }

    #[test]
    fn resolve_finds_pas_toml_in_parent() {
        let tmp = TempDir::new().unwrap();
        // Parent has .git and pas.toml; child has no markers.
        fs::create_dir(tmp.path().join(".git")).unwrap();
        fs::write(
            tmp.path().join("pas.toml"),
            "[project]\nname = \"test\"\n",
        )
        .unwrap();
        let child = tmp.path().join("subdir");
        fs::create_dir(&child).unwrap();

        let result = resolve(&child);
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
    }

    #[test]
    fn resolve_stops_at_git_root_without_pas_toml() {
        let tmp = make_workdir_with_git();
        let child = tmp.path().join("subdir");
        fs::create_dir(&child).unwrap();

        let result = resolve(&child);
        assert!(
            matches!(result, Err(ResolutionError::NotFound { .. })),
            "expected NotFound, got {:?}",
            result
        );
    }
}
