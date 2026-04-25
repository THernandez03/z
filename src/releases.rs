use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

pub const INDEX_URL: &str = "https://ziglang.org/download/index.json";

#[derive(Debug, Clone)]
pub struct ReleaseInfo {
    pub version: String,
    pub tarball_url: String,
    pub is_master: bool,
}

/// Fetch and list all available remote versions.
pub fn list_remote() -> Result<()> {
    let versions = fetch_versions()?;
    println!("Available Zig versions:");
    for v in &versions {
        if v.is_master {
            println!("  master  ({})", v.version);
        } else {
            println!("  {}", v.version);
        }
    }
    Ok(())
}

/// Fetch all versions from the Zig download index.
pub fn fetch_versions() -> Result<Vec<ReleaseInfo>> {
    let client = Client::new();
    let resp: Value = client
        .get(INDEX_URL)
        .header("User-Agent", "z-zig-version-manager")
        .send()
        .context("Failed to fetch Zig download index")?  
        .json()
        .context("Failed to parse Zig download index")?;

    let map = resp.as_object().context("Expected JSON object")?;
    let target = crate::arch::target();
    let ext = crate::arch::archive_ext();

    let mut results = Vec::new();

    for (key, entry) in map {
        let is_master = key == "master";
        let version = entry["version"]
            .as_str()
            .unwrap_or(key)
            .to_string();

        if let Some(platform_info) = entry.get(target) {
            if let Some(url) = platform_info.get("tarball").and_then(|v| v.as_str()) {
                results.push(ReleaseInfo {
                    version: if is_master { key.clone() } else { version },
                    tarball_url: url.to_string(),
                    is_master,
                });
            }
        }
    }

    // Sort: master first, then descending semver
    results.sort_by(|a, b| {
        if a.is_master {
            return std::cmp::Ordering::Less;
        }
        if b.is_master {
            return std::cmp::Ordering::Greater;
        }
        // Compare version strings (simple lexicographic on semver parts)
        b.version.cmp(&a.version)
    });

    Ok(results)
}

/// Resolve a version label to a ReleaseInfo.
/// Accepts: "master", a full version like "0.13.0", or a prefix like "0.13".
pub fn resolve(version_str: &str) -> Result<ReleaseInfo> {
    let versions = fetch_versions()?;

    let label = version_str.trim_start_matches('v');

    // Exact match on key or version field
    for v in &versions {
        if v.version == label || (v.is_master && label == "master") {
            return Ok(v.clone());
        }
    }

    // Prefix match (e.g. "0.13" matches "0.13.0")
    for v in &versions {
        if v.version.starts_with(label) {
            return Ok(v.clone());
        }
    }

    anyhow::bail!("No Zig release found matching '{}'", version_str)
}
