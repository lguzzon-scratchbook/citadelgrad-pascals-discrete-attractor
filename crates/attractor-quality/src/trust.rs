use std::collections::HashMap;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrustError {
    #[error("manifest is not trusted")]
    Untrusted,
    #[error("trust store is corrupted: {0}")]
    CorruptedStore(String),
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct TrustStore {
    entries: HashMap<String, TrustEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustEntry {
    pub path: String,
    pub blake3_hash: String,
    pub source: String,
    pub added_at: String,
}

fn trust_store_path() -> PathBuf {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".config")
        });
    base.join("pas").join("trusted.json")
}

fn make_key(path: &Path, blake3: &str) -> String {
    format!("{}\0{}", path.display(), blake3)
}

fn load_store() -> Result<TrustStore, TrustError> {
    let p = trust_store_path();
    if !p.exists() {
        return Ok(TrustStore::default());
    }
    let content = std::fs::read_to_string(&p)
        .map_err(|e| TrustError::CorruptedStore(e.to_string()))?;
    serde_json::from_str(&content).map_err(|e| TrustError::CorruptedStore(e.to_string()))
}

fn save_store(store: &TrustStore) -> Result<(), TrustError> {
    let p = trust_store_path();
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| TrustError::CorruptedStore(e.to_string()))?;
    }
    let content = serde_json::to_string_pretty(store)
        .map_err(|e| TrustError::CorruptedStore(e.to_string()))?;

    let dir = p.parent().unwrap_or(Path::new("."));
    let tmp = tempfile::Builder::new()
        .prefix("trusted")
        .suffix(".json.tmp")
        .tempfile_in(dir)
        .map_err(|e| TrustError::CorruptedStore(e.to_string()))?;

    std::fs::write(tmp.path(), &content)
        .map_err(|e| TrustError::CorruptedStore(e.to_string()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(tmp.path(), std::fs::Permissions::from_mode(0o600))
            .map_err(|e| TrustError::CorruptedStore(e.to_string()))?;
    }

    tmp.persist(&p)
        .map_err(|e| TrustError::CorruptedStore(e.to_string()))?;

    Ok(())
}

pub fn is_trusted(path: &Path, blake3: &str) -> bool {
    if std::env::var("PAS_TRUST_THIS").as_deref() == Ok("1") {
        return true;
    }
    if std::env::var("PAS_AGENT").as_deref() == Ok("1") {
        return true;
    }
    let store = match load_store() {
        Ok(s) => s,
        Err(_) => return false,
    };
    store.entries.contains_key(&make_key(path, blake3))
}

pub fn add_trust(path: &Path, blake3: &str, source: &str) -> Result<(), TrustError> {
    let mut store = load_store()?;
    store.entries.insert(
        make_key(path, blake3),
        TrustEntry {
            path: path.to_string_lossy().to_string(),
            blake3_hash: blake3.to_string(),
            source: source.to_string(),
            added_at: chrono::Utc::now().to_rfc3339(),
        },
    );
    save_store(&store)
}

pub fn remove_trust(path: &Path, blake3: &str) -> Result<(), TrustError> {
    let mut store = load_store()?;
    store.entries.remove(&make_key(path, blake3));
    save_store(&store)
}

pub fn list_trusted() -> Result<Vec<TrustEntry>, TrustError> {
    let store = load_store()?;
    Ok(store.entries.into_values().collect())
}

pub fn prompt_and_add(path: &Path, blake3: &str) -> Result<bool, TrustError> {
    if !std::io::stdin().is_terminal()
        || std::env::var("PAS_NON_INTERACTIVE").as_deref() == Ok("1")
        || std::env::var("PAS_AGENT").as_deref() == Ok("1")
    {
        return Ok(false);
    }

    eprintln!("pas: trust this manifest?");
    eprintln!("  path: {}", path.display());
    eprintln!("  hash: {}", &blake3[..blake3.len().min(16)]);
    eprint!("Trust? [y/N] ");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| TrustError::CorruptedStore(e.to_string()))?;

    if input.trim().to_lowercase() == "y" {
        add_trust(path, blake3, "prompt")?;
        return Ok(true);
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use tempfile::TempDir;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn trust_roundtrip() {
        let _guard = TEST_LOCK.lock().unwrap();
        let tmp = TempDir::new().unwrap();
        std::env::set_var("XDG_CONFIG_HOME", tmp.path());

        let path = Path::new("/tmp/test/pas.toml");
        let hash = "abc123def456789";

        assert!(!is_trusted(path, hash));
        add_trust(path, hash, "test").unwrap();
        assert!(is_trusted(path, hash));
        remove_trust(path, hash).unwrap();
        assert!(!is_trusted(path, hash));

        std::env::remove_var("XDG_CONFIG_HOME");
    }

    #[test]
    fn pas_trust_this_env_bypasses_check() {
        let _guard = TEST_LOCK.lock().unwrap();
        std::env::set_var("PAS_TRUST_THIS", "1");
        assert!(is_trusted(Path::new("/any/path"), "any_hash"));
        std::env::remove_var("PAS_TRUST_THIS");
    }

    #[test]
    fn list_trusted_returns_entries() {
        let _guard = TEST_LOCK.lock().unwrap();
        let tmp = TempDir::new().unwrap();
        std::env::set_var("XDG_CONFIG_HOME", tmp.path());

        add_trust(Path::new("/a/pas.toml"), "hash1", "test").unwrap();
        add_trust(Path::new("/b/pas.toml"), "hash2", "test").unwrap();
        let entries = list_trusted().unwrap();
        assert_eq!(entries.len(), 2);

        std::env::remove_var("XDG_CONFIG_HOME");
    }
}
