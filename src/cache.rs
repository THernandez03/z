use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// Root cache directory: $`Z_CACHE_DIR` or $`Z_PREFIX/versions`, defaulting to ~/.z/versions
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
        anyhow::bail!("Version '{version}' is not cached. Run `z {version}` to install it.")
    }
}

/// Remove a cached version.
pub fn remove(version: &str) -> Result<()> {
    let dir = version_dir(version);
    if dir.exists() {
        fs::remove_dir_all(&dir)
            .with_context(|| format!("Failed to remove cached version '{version}'"))?;
        println!("Removed {version}");
    } else {
        println!("Version '{version}' is not cached.");
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
                .with_context(|| format!("Failed to remove '{name}'"))?;
            println!("Removed {name}");
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::Mutex;

    // Serialize all tests that mutate environment variables to avoid races.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    /// Run `f` with `Z_CACHE_DIR` pointing to a fresh temp dir.
    /// Acquires `ENV_LOCK` for the duration.
    fn with_temp_cache<F: FnOnce(&std::path::Path)>(f: F) {
        let _guard = ENV_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().expect("tempdir");
        std::env::set_var("Z_CACHE_DIR", dir.path());
        f(dir.path());
        std::env::remove_var("Z_CACHE_DIR");
    }

    #[test]
    fn cache_dir_respects_env_var() {
        let _guard = ENV_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().expect("tempdir");
        std::env::set_var("Z_CACHE_DIR", dir.path());
        let result = cache_dir();
        std::env::remove_var("Z_CACHE_DIR");
        assert_eq!(result, dir.path());
    }

    #[test]
    fn version_dir_is_under_cache_dir() {
        let _guard = ENV_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().expect("tempdir");
        std::env::set_var("Z_CACHE_DIR", dir.path());
        let vdir = version_dir("0.13.0");
        std::env::remove_var("Z_CACHE_DIR");
        assert_eq!(vdir, dir.path().join("0.13.0"));
    }

    #[test]
    fn zig_binary_points_into_version_dir() {
        let _guard = ENV_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().expect("tempdir");
        std::env::set_var("Z_CACHE_DIR", dir.path());
        let bin = zig_binary("0.13.0");
        std::env::remove_var("Z_CACHE_DIR");
        assert!(bin.starts_with(dir.path()));
        let name = bin.file_name().unwrap().to_string_lossy();
        assert!(name == "zig" || name == "zig.exe");
    }

    #[test]
    fn is_cached_returns_false_when_binary_missing() {
        with_temp_cache(|_| {
            assert!(!is_cached("nonexistent-version"));
        });
    }

    #[test]
    fn is_cached_returns_true_when_binary_present() {
        with_temp_cache(|base| {
            let vdir = base.join("0.13.0");
            fs::create_dir_all(&vdir).unwrap();
            fs::write(vdir.join("zig"), b"fake").unwrap();
            assert!(is_cached("0.13.0"));
        });
    }

    #[test]
    fn which_errors_when_not_cached() {
        with_temp_cache(|_| {
            assert!(which("nope").is_err());
        });
    }

    #[test]
    fn which_returns_path_when_cached() {
        with_temp_cache(|base| {
            let vdir = base.join("0.13.0");
            fs::create_dir_all(&vdir).unwrap();
            fs::write(vdir.join("zig"), b"fake").unwrap();
            let result = which("0.13.0");
            assert!(result.is_ok());
        });
    }

    #[test]
    fn remove_removes_existing_version() {
        with_temp_cache(|base| {
            let vdir = base.join("0.13.0");
            fs::create_dir_all(&vdir).unwrap();
            fs::write(vdir.join("zig"), b"fake").unwrap();
            assert!(remove("0.13.0").is_ok());
            assert!(!vdir.exists());
        });
    }

    #[test]
    fn remove_is_ok_when_version_not_cached() {
        with_temp_cache(|_| {
            // Should not error
            assert!(remove("does-not-exist").is_ok());
        });
    }

    #[test]
    fn cached_versions_empty_when_cache_missing() {
        with_temp_cache(|base| {
            // Remove the temp dir to simulate no cache dir
            fs::remove_dir_all(base).unwrap();
            let result = cached_versions();
            assert!(result.is_ok());
            assert!(result.unwrap().is_empty());
        });
    }

    #[test]
    fn cached_versions_lists_dirs_sorted_descending() {
        with_temp_cache(|base| {
            for v in &["0.11.0", "0.13.0", "0.12.0"] {
                let vdir = base.join(v);
                fs::create_dir_all(&vdir).unwrap();
            }
            let versions = cached_versions().unwrap();
            assert_eq!(versions, vec!["0.13.0", "0.12.0", "0.11.0"]);
        });
    }

    #[test]
    fn cached_versions_ignores_files() {
        with_temp_cache(|base| {
            fs::write(base.join("some-file.txt"), b"").unwrap();
            let vdir = base.join("0.13.0");
            fs::create_dir_all(&vdir).unwrap();
            let versions = cached_versions().unwrap();
            assert_eq!(versions, vec!["0.13.0"]);
        });
    }

    #[test]
    fn prune_removes_non_active_versions() {
        let _guard = ENV_LOCK.lock().unwrap();
        let prefix_dir = tempfile::tempdir().expect("tempdir");
        std::env::set_var("Z_PREFIX", prefix_dir.path());
        std::env::remove_var("Z_CACHE_DIR");

        let cache = prefix_dir.path().join("versions");
        fs::create_dir_all(&cache).unwrap();

        for v in &["0.11.0", "0.12.0", "0.13.0"] {
            fs::create_dir_all(cache.join(v)).unwrap();
        }

        // Write active marker
        fs::write(prefix_dir.path().join(".active"), "0.13.0").unwrap();

        prune().unwrap();

        assert!(!cache.join("0.11.0").exists());
        assert!(!cache.join("0.12.0").exists());
        assert!(cache.join("0.13.0").exists());

        std::env::remove_var("Z_PREFIX");
    }
}
