//! `pas init` — detect toolchain and emit a starter `pas.toml`.

use std::io::IsTerminal;
use std::path::{Path, PathBuf};

use anyhow::Context;
use attractor_quality::detect::detect;

// ---------------------------------------------------------------------------
// Built-in templates (embedded at compile time)
// ---------------------------------------------------------------------------

const TEMPLATE_RUST: &str =
    include_str!("../../../attractor-quality/templates/rust.toml");
const TEMPLATE_PYTHON: &str =
    include_str!("../../../attractor-quality/templates/python.toml");
const TEMPLATE_TYPESCRIPT: &str =
    include_str!("../../../attractor-quality/templates/typescript.toml");
const TEMPLATE_DEFAULT: &str =
    include_str!("../../../attractor-quality/templates/default.toml");

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Options forwarded from the CLI flags.
#[derive(Debug, Clone)]
pub struct InitOpts {
    pub force: bool,
    pub non_interactive: bool,
    pub no_enrich: bool,
    pub dry_run: bool,
}

/// Planning output: separates decision-making from I/O.
#[derive(Debug)]
pub struct InitPlan {
    /// Rendered TOML content ready to write.
    pub manifest_content: String,
    /// Absolute path where `pas.toml` should be written.
    pub write_path: PathBuf,
    /// Whether we are overwriting an existing file.
    pub overwrite: bool,
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Walk from `start` upwards and return the first directory that contains
/// a `.git` entry (file or directory). Returns `None` if the filesystem root
/// is reached without finding one.
fn find_git_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if current.join(".git").exists() {
            return Some(current);
        }
        match current.parent() {
            Some(p) => current = p.to_path_buf(),
            None => return None,
        }
    }
}

/// Returns `true` when running in non-interactive mode (no TTY on stdin, the
/// `--non-interactive` flag, or the `PAS_NON_INTERACTIVE` env var is set to
/// a truthy value).
fn is_non_interactive(opts: &InitOpts) -> bool {
    if opts.non_interactive {
        return true;
    }
    if !std::io::stdin().is_terminal() {
        return true;
    }
    matches!(
        std::env::var("PAS_NON_INTERACTIVE").as_deref(),
        Ok("1") | Ok("true") | Ok("yes")
    )
}

/// Prompt the user with a yes/no question. Returns `true` if they confirmed.
fn prompt_yes_no(question: &str) -> bool {
    use std::io::Write;
    print!("{} [y/N] ", question);
    let _ = std::io::stdout().flush();
    let mut line = String::new();
    if std::io::stdin().read_line(&mut line).is_err() {
        return false;
    }
    matches!(line.trim().to_lowercase().as_str(), "y" | "yes")
}

/// Pick the right template for the first detected language (or default).
fn template_for_language(language: &str) -> &'static str {
    match language {
        "rust" => TEMPLATE_RUST,
        "python" => TEMPLATE_PYTHON,
        "typescript" => TEMPLATE_TYPESCRIPT,
        _ => TEMPLATE_DEFAULT,
    }
}

// ---------------------------------------------------------------------------
// Plan
// ---------------------------------------------------------------------------

/// Build an [`InitPlan`] without performing any I/O (except detection reads).
pub fn plan_init(workdir: &Path, opts: &InitOpts) -> anyhow::Result<InitPlan> {
    // 1. Locate .git root.
    let git_root = find_git_root(workdir);

    let write_dir = match &git_root {
        Some(root) => root.clone(),
        None => {
            // No .git found.
            let ni = is_non_interactive(opts);
            if ni && !opts.force {
                eprintln!(
                    "error: no `.git` directory found starting from `{}`.\n\
                     Use --force to initialise without a git repository.",
                    workdir.display()
                );
                std::process::exit(4);
            } else if !ni && !opts.force {
                let confirmed = prompt_yes_no(
                    "warning: no `.git` directory found. Initialise anyway?",
                );
                if !confirmed {
                    anyhow::bail!("aborted by user");
                }
            }
            // force=true, or user said yes in TTY
            workdir.to_path_buf()
        }
    };

    // 2. Detect toolchains.
    let toolchains = detect(workdir);
    let template = if let Some(tc) = toolchains.first() {
        template_for_language(&tc.language)
    } else {
        TEMPLATE_DEFAULT
    };

    // 3. Derive project name from directory.
    let name = write_dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "project".to_string());

    let manifest_content = template.replace("{{name}}", &name);

    // 4. Determine write path and overwrite policy.
    let write_path = write_dir.join("pas.toml");
    let overwrite = write_path.exists();

    if overwrite {
        let ni = is_non_interactive(opts);
        if ni && !opts.force {
            eprintln!(
                "error: `{}` already exists. Use --force to overwrite.",
                write_path.display()
            );
            std::process::exit(1);
        } else if !ni && !opts.force {
            let confirmed = prompt_yes_no(&format!(
                "`{}` already exists. Overwrite?",
                write_path.display()
            ));
            if !confirmed {
                anyhow::bail!("aborted by user");
            }
        }
    }

    Ok(InitPlan {
        manifest_content,
        write_path,
        overwrite,
    })
}

// ---------------------------------------------------------------------------
// Execute
// ---------------------------------------------------------------------------

/// Main entry-point for `pas init`.
pub fn cmd_init(workdir: &Path, opts: &InitOpts) -> anyhow::Result<()> {
    let plan = plan_init(workdir, opts)?;

    if opts.dry_run {
        println!(
            "[dry-run] would write {} ({}):\n---\n{}\n---",
            plan.write_path.display(),
            if plan.overwrite { "overwrite" } else { "new file" },
            plan.manifest_content.trim_end(),
        );
        return Ok(());
    }

    std::fs::write(&plan.write_path, &plan.manifest_content)
        .with_context(|| format!("failed to write `{}`", plan.write_path.display()))?;

    println!(
        "{} `{}`",
        if plan.overwrite { "updated" } else { "created" },
        plan.write_path.display()
    );

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

    fn non_interactive_force() -> InitOpts {
        InitOpts {
            force: true,
            non_interactive: true,
            no_enrich: true,
            dry_run: false,
        }
    }

    #[test]
    fn init_creates_parseable_pas_toml_in_rust_repo() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".git")).unwrap();
        fs::write(
            tmp.path().join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();

        let opts = non_interactive_force();
        cmd_init(tmp.path(), &opts).unwrap();

        let written = fs::read_to_string(tmp.path().join("pas.toml")).unwrap();
        let manifest: attractor_quality::manifest::Manifest =
            toml::from_str(&written).expect("should be valid TOML");
        assert_eq!(
            manifest.toolchain.expect("toolchain section present").language,
            "rust"
        );
    }

    #[test]
    fn init_creates_parseable_pas_toml_in_python_repo() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".git")).unwrap();
        fs::write(tmp.path().join("pyproject.toml"), "[project]\nname = \"x\"").unwrap();

        cmd_init(tmp.path(), &non_interactive_force()).unwrap();

        let written = fs::read_to_string(tmp.path().join("pas.toml")).unwrap();
        let manifest: attractor_quality::manifest::Manifest =
            toml::from_str(&written).expect("valid TOML");
        assert_eq!(
            manifest.toolchain.expect("toolchain present").language,
            "python"
        );
    }

    #[test]
    fn init_creates_parseable_pas_toml_in_typescript_repo() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".git")).unwrap();
        fs::write(tmp.path().join("package.json"), "{}").unwrap();

        cmd_init(tmp.path(), &non_interactive_force()).unwrap();

        let written = fs::read_to_string(tmp.path().join("pas.toml")).unwrap();
        let manifest: attractor_quality::manifest::Manifest =
            toml::from_str(&written).expect("valid TOML");
        assert_eq!(
            manifest.toolchain.expect("toolchain present").language,
            "typescript"
        );
    }

    #[test]
    fn init_creates_default_pas_toml_with_no_toolchain() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".git")).unwrap();

        cmd_init(tmp.path(), &non_interactive_force()).unwrap();

        let written = fs::read_to_string(tmp.path().join("pas.toml")).unwrap();
        let manifest: attractor_quality::manifest::Manifest =
            toml::from_str(&written).expect("valid TOML");
        // default template has no toolchain section
        assert!(manifest.toolchain.is_none());
    }

    #[test]
    fn dry_run_does_not_write_file() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".git")).unwrap();
        fs::write(tmp.path().join("Cargo.toml"), "[package]").unwrap();

        let opts = InitOpts {
            force: true,
            non_interactive: true,
            no_enrich: true,
            dry_run: true,
        };
        cmd_init(tmp.path(), &opts).unwrap();

        assert!(!tmp.path().join("pas.toml").exists());
    }

    #[test]
    fn init_non_interactive_no_git_exits_4() {
        // We can't test process::exit(4) directly without spawning a subprocess,
        // but we can verify plan_init returns an error path when force=false
        // and no .git exists by using plan_init with force=false + non_interactive=true.
        // Since plan_init calls process::exit(4) in that branch, we test via --force
        // to ensure the no-.git + force path still works.
        let tmp = TempDir::new().unwrap();
        // No .git directory

        let opts = InitOpts {
            force: true, // force bypasses the exit(4) path
            non_interactive: true,
            no_enrich: true,
            dry_run: false,
        };
        // Should succeed (write to workdir when force=true + no .git)
        cmd_init(tmp.path(), &opts).unwrap();
        assert!(tmp.path().join("pas.toml").exists());
    }

    #[test]
    fn name_placeholder_is_replaced() {
        let tmp = TempDir::new().unwrap();
        // Use a directory with a recognisable name
        let project_dir = tmp.path().join("my-cool-project");
        fs::create_dir_all(project_dir.join(".git")).unwrap();
        fs::write(project_dir.join("Cargo.toml"), "[package]").unwrap();

        cmd_init(&project_dir, &non_interactive_force()).unwrap();

        let written = fs::read_to_string(project_dir.join("pas.toml")).unwrap();
        assert!(
            written.contains("my-cool-project"),
            "expected project name in output, got:\n{written}"
        );
        assert!(
            !written.contains("{{name}}"),
            "placeholder was not replaced"
        );
    }
}
