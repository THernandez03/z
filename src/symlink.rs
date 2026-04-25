use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// Installation prefix: $`Z_PREFIX` or ~/.z
pub fn prefix() -> PathBuf {
    if let Ok(p) = std::env::var("Z_PREFIX") {
        return PathBuf::from(p);
    }
    dirs_next::home_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join(".z")
}

/// The bin directory where the active `zig` symlink lives.
pub fn bin_dir() -> PathBuf {
    prefix().join("bin")
}

/// Activate a cached version by creating/updating a symlink.
pub fn activate(version: &str) -> Result<()> {
    let bin = bin_dir();
    fs::create_dir_all(&bin).context("Failed to create bin directory")?;

    let zig_src = crate::cache::zig_binary(version);

    #[cfg(target_os = "windows")]
    let link_path = bin.join("zig.exe");
    #[cfg(not(target_os = "windows"))]
    let link_path = bin.join("zig");

    // Remove existing link/file
    if link_path.exists() || link_path.symlink_metadata().is_ok() {
        fs::remove_file(&link_path).ok();
    }

    #[cfg(unix)]
    std::os::unix::fs::symlink(&zig_src, &link_path).with_context(|| {
        format!(
            "Failed to create symlink {} -> {}",
            link_path.display(),
            zig_src.display()
        )
    })?;

    #[cfg(windows)]
    std::os::windows::fs::symlink_file(&zig_src, &link_path).with_context(|| {
        format!(
            "Failed to create symlink {} -> {}",
            link_path.display(),
            zig_src.display()
        )
    })?;

    // Write active version marker
    let marker = prefix().join(".active");
    fs::write(&marker, version).context("Failed to write active version marker")?;

    Ok(())
}

/// Read the currently active version from the marker file.
pub fn active_version() -> Option<String> {
    let marker = prefix().join(".active");
    fs::read_to_string(marker)
        .ok()
        .map(|s| s.trim().to_string())
}

/// Remove the active zig symlink (does not remove cache).
pub fn uninstall() -> Result<()> {
    let bin = bin_dir();

    #[cfg(target_os = "windows")]
    let link_path = bin.join("zig.exe");
    #[cfg(not(target_os = "windows"))]
    let link_path = bin.join("zig");

    if link_path.exists() || link_path.symlink_metadata().is_ok() {
        fs::remove_file(&link_path).context("Failed to remove zig symlink")?;
        println!("Removed active Zig installation.");
    } else {
        println!("No active Zig installation found.");
    }

    let marker = prefix().join(".active");
    fs::remove_file(marker).ok();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_temp_prefix<F: FnOnce(&std::path::Path)>(f: F) {
        let _guard = ENV_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().expect("tempdir");
        std::env::set_var("Z_PREFIX", dir.path());
        std::env::remove_var("Z_CACHE_DIR");
        f(dir.path());
        std::env::remove_var("Z_PREFIX");
    }

    #[test]
    fn prefix_respects_env_var() {
        let _guard = ENV_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().expect("tempdir");
        std::env::set_var("Z_PREFIX", dir.path());
        let result = prefix();
        std::env::remove_var("Z_PREFIX");
        assert_eq!(result, dir.path());
    }

    #[test]
    fn bin_dir_is_under_prefix() {
        let _guard = ENV_LOCK.lock().unwrap();
        let dir = tempfile::tempdir().expect("tempdir");
        std::env::set_var("Z_PREFIX", dir.path());
        let b = bin_dir();
        std::env::remove_var("Z_PREFIX");
        assert_eq!(b, dir.path().join("bin"));
    }

    #[test]
    fn active_version_returns_none_when_marker_missing() {
        with_temp_prefix(|_| {
            assert_eq!(active_version(), None);
        });
    }

    #[test]
    fn active_version_reads_marker_file() {
        with_temp_prefix(|base| {
            fs::write(base.join(".active"), "0.13.0").unwrap();
            assert_eq!(active_version(), Some("0.13.0".to_string()));
        });
    }

    #[test]
    fn active_version_trims_whitespace() {
        with_temp_prefix(|base| {
            fs::write(base.join(".active"), "  0.13.0\n").unwrap();
            assert_eq!(active_version(), Some("0.13.0".to_string()));
        });
    }

    #[cfg(unix)]
    #[test]
    fn activate_creates_symlink_and_marker() {
        with_temp_prefix(|base| {
            std::env::set_var("Z_CACHE_DIR", base.join("versions"));
            let vdir = base.join("versions").join("0.13.0");
            fs::create_dir_all(&vdir).unwrap();
            let fake_bin = vdir.join("zig");
            fs::write(&fake_bin, b"#!/bin/sh").unwrap();

            activate("0.13.0").unwrap();

            let link = base.join("bin").join("zig");
            assert!(link.symlink_metadata().is_ok(), "symlink should exist");
            assert_eq!(active_version(), Some("0.13.0".to_string()));
            std::env::remove_var("Z_CACHE_DIR");
        });
    }

    #[cfg(unix)]
    #[test]
    fn activate_replaces_existing_symlink() {
        with_temp_prefix(|base| {
            std::env::set_var("Z_CACHE_DIR", base.join("versions"));
            for v in &["0.12.0", "0.13.0"] {
                let vdir = base.join("versions").join(v);
                fs::create_dir_all(&vdir).unwrap();
                fs::write(vdir.join("zig"), b"#!/bin/sh").unwrap();
            }

            activate("0.12.0").unwrap();
            activate("0.13.0").unwrap();

            assert_eq!(active_version(), Some("0.13.0".to_string()));
            std::env::remove_var("Z_CACHE_DIR");
        });
    }

    #[cfg(unix)]
    #[test]
    fn uninstall_removes_symlink_and_marker() {
        with_temp_prefix(|base| {
            std::env::set_var("Z_CACHE_DIR", base.join("versions"));
            let vdir = base.join("versions").join("0.13.0");
            fs::create_dir_all(&vdir).unwrap();
            fs::write(vdir.join("zig"), b"#!/bin/sh").unwrap();

            activate("0.13.0").unwrap();
            uninstall().unwrap();

            let link = base.join("bin").join("zig");
            assert!(!link.exists());
            assert_eq!(active_version(), None);
            std::env::remove_var("Z_CACHE_DIR");
        });
    }

    #[test]
    fn uninstall_is_ok_when_nothing_installed() {
        with_temp_prefix(|_| {
            assert!(uninstall().is_ok());
        });
    }
}
