use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// Root cache directory: $Z_CACHE_DIR or $Z_PREFIX/versions, defaulting to ~/.z/versions
pub fn cache_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("Z_CACHE_DIR") {
        return PathBuf::from(dir);
    }
    let prefix = crate::symlink::prefix();
    prefix.join("versions")
}

/// Path to a specific cached version directory.
pub fn version_dir(version: &str) -> PathBuf {
    cache_dir().join(version)
}

/// Path to the zig binary inside a cached version.
pub fn zig_binary(version: &str) -> PathBuf {
    let dir = version_dir(version);
    #[cfg(target_os = "windows")]
    return dir.join("zig.exe");
    #[cfg(not(target_os = "windows"))]
    return dir.join("zig");
}

/// Check whether a version is already cached.
pub fn is_cached(version: &str) -> bool {
    zig_binary(version).exists()
}

/// Return the path to the zig binary, error if not cached.
pub fn which(version: &str) -> Result<PathBuf> {
    let path = zig_binary(version);
    if path.exists() {
        Ok(path)
    } else {
        anyhow::bail!("Version '{}' is not cached. Run `z install {}` first.", version, version)
    }
}

/// Remove a cached version.
pub fn remove(version: &str) -> Result<()> {
    let dir = version_dir(version);
    if dir.exists() {
        fs::remove_dir_all(&dir)
            .with_context(|| format!("Failed to remove cached version '{}'", version))?;
        println!("Removed {}", version);
    } else {
        println!("Version '{}' is not cached.", version);
    }
    Ok(())
}

/// Remove all cached versions except the currently active one.
pub fn prune() -> Result<()> {
    let active = crate::symlink::active_version();
    let dir = cache_dir();

    if !dir.exists() {
        println!("Cache directory does not exist.");
        return Ok(());
    }

    for entry in fs::read_dir(&dir).context("Failed to read cache directory")? {
        let entry = entry?;
        let name = entry.file_name().into_string().unwrap_or_default();
        if Some(&name) == active.as_ref() {
            continue;
        }
        if entry.path().is_dir() {
            fs::remove_dir_all(entry.path())
                .with_context(|| format!("Failed to remove '{}'", name))?;
            println!("Removed {}", name);
        }
    }
    Ok(())
}

/// Return all locally cached version names.
pub fn cached_versions() -> Result<Vec<String>> {
    let dir = cache_dir();
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut versions = vec![];
    for entry in fs::read_dir(&dir).context("Failed to read cache directory")? {
        let entry = entry?;
        if entry.path().is_dir() {
            let name = entry.file_name().into_string().unwrap_or_default();
            if !name.is_empty() {
                versions.push(name);
            }
        }
    }
    versions.sort_by(|a, b| b.cmp(a));
    Ok(versions)
}
