use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// Installation prefix: $Z_PREFIX or ~/.z
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
    std::os::unix::fs::symlink(&zig_src, &link_path)
        .with_context(|| format!("Failed to create symlink {:?} -> {:?}", link_path, zig_src))?;

    #[cfg(windows)]
    std::os::windows::fs::symlink_file(&zig_src, &link_path)
        .with_context(|| format!("Failed to create symlink {:?} -> {:?}", link_path, zig_src))?;

    // Write active version marker
    let marker = prefix().join(".active");
    fs::write(&marker, version).context("Failed to write active version marker")?;

    Ok(())
}

/// Read the currently active version from the marker file.
pub fn active_version() -> Option<String> {
    let marker = prefix().join(".active");
    fs::read_to_string(marker).ok().map(|s| s.trim().to_string())
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
