use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::Value;

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
    let _ext = crate::arch::archive_ext();

    let mut results = Vec::new();

    for (key, entry) in map {
        let is_master = key == "master";
        let version = entry["version"].as_str().unwrap_or(key).to_string();

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

/// Resolve a version label to a `ReleaseInfo`.
///
/// Accepts: `master`, a full version like `0.13.0`, a prefix like `0.13`, a
/// leading `v` (`v0.13.0`), or the convenience aliases `lts`, `latest`,
/// `canary`, and `next`.
pub fn resolve(version_str: &str) -> Result<ReleaseInfo> {
    let versions = fetch_versions()?;
    resolve_from(version_str, &versions)
}

/// Inner resolution logic that operates on a pre-built slice, allowing tests
/// to bypass the network.
fn resolve_from(version_str: &str, versions: &[ReleaseInfo]) -> Result<ReleaseInfo> {
    let label = version_str.trim_start_matches('v');

    // Handle convenience aliases
    match label {
        // canary / latest / next  →  master (bleeding-edge nightly build)
        "canary" | "latest" | "next" => {
            return versions
                .iter()
                .find(|v| v.is_master)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("No master/nightly release found"));
        }
        // lts / stable  →  highest stable (non-master) release
        "lts" | "stable" => {
            return versions
                .iter()
                .find(|v| !v.is_master)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("No stable release found"));
        }
        _ => {}
    }

    // Exact match on version field or project-native key (e.g. "master")
    for v in versions {
        if v.version == label || (v.is_master && label == "master") {
            return Ok(v.clone());
        }
    }

    // Prefix match (e.g. "0.13" matches "0.13.0")
    for v in versions {
        if v.version.starts_with(label) {
            return Ok(v.clone());
        }
    }

    anyhow::bail!("No Zig release found matching '{version_str}'")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_release(version: &str, is_master: bool) -> ReleaseInfo {
        ReleaseInfo {
            version: version.to_string(),
            tarball_url: format!("https://example.com/zig-{version}.tar.xz"),
            is_master,
        }
    }

    /// Resolve within a pre-built list, bypassing network.
    fn resolve_in(versions: &[ReleaseInfo], label: &str) -> anyhow::Result<ReleaseInfo> {
        resolve_from(label, versions)
    }

    #[test]
    fn resolve_exact_version() {
        let releases = [make_release("0.13.0", false), make_release("0.12.0", false)];
        let r = resolve_in(&releases, "0.13.0").unwrap();
        assert_eq!(r.version, "0.13.0");
    }

    #[test]
    fn resolve_master_label() {
        let releases = [make_release("master", true), make_release("0.13.0", false)];
        let r = resolve_in(&releases, "master").unwrap();
        assert!(r.is_master);
    }

    #[test]
    fn resolve_prefix_match() {
        let releases = [make_release("0.13.0", false), make_release("0.12.0", false)];
        let r = resolve_in(&releases, "0.13").unwrap();
        assert_eq!(r.version, "0.13.0");
    }

    #[test]
    fn resolve_strips_leading_v() {
        let releases = [make_release("0.13.0", false)];
        let r = resolve_in(&releases, "v0.13.0").unwrap();
        assert_eq!(r.version, "0.13.0");
    }

    #[test]
    fn resolve_errors_on_unknown_version() {
        let releases = [make_release("0.13.0", false)];
        assert!(resolve_in(&releases, "99.99.99").is_err());
    }

    #[test]
    fn resolve_errors_on_empty_list() {
        assert!(resolve_in(&[], "0.13.0").is_err());
    }

    // --- alias tests ---

    #[test]
    fn alias_latest_resolves_to_master() {
        let releases = [make_release("master", true), make_release("0.13.0", false)];
        let r = resolve_in(&releases, "latest").unwrap();
        assert!(r.is_master);
    }

    #[test]
    fn alias_canary_resolves_to_master() {
        let releases = [make_release("master", true), make_release("0.13.0", false)];
        let r = resolve_in(&releases, "canary").unwrap();
        assert!(r.is_master);
    }

    #[test]
    fn alias_next_resolves_to_master() {
        let releases = [make_release("master", true), make_release("0.13.0", false)];
        let r = resolve_in(&releases, "next").unwrap();
        assert!(r.is_master);
    }

    #[test]
    fn alias_lts_resolves_to_first_stable() {
        // master comes first in the sorted list; lts should skip it
        let releases = [
            make_release("master", true),
            make_release("0.13.0", false),
            make_release("0.12.0", false),
        ];
        let r = resolve_in(&releases, "lts").unwrap();
        assert!(!r.is_master);
        assert_eq!(r.version, "0.13.0");
    }

    #[test]
    fn alias_lts_errors_when_no_stable() {
        let releases = [make_release("master", true)];
        assert!(resolve_in(&releases, "lts").is_err());
    }

    #[test]
    fn alias_latest_errors_when_no_master() {
        let releases = [make_release("0.13.0", false)];
        assert!(resolve_in(&releases, "latest").is_err());
    }

    #[test]
    fn release_info_clone() {
        let r = make_release("0.13.0", false);
        let r2 = r.clone();
        assert_eq!(r.version, r2.version);
        assert_eq!(r.tarball_url, r2.tarball_url);
        assert_eq!(r.is_master, r2.is_master);
    }

    #[test]
    fn index_url_is_https() {
        assert!(INDEX_URL.starts_with("https://"));
    }

    #[test]
    fn sort_master_first() {
        let mut releases = [
            make_release("0.13.0", false),
            make_release("master", true),
            make_release("0.12.0", false),
        ];
        releases.sort_by(|a, b| {
            if a.is_master {
                return std::cmp::Ordering::Less;
            }
            if b.is_master {
                return std::cmp::Ordering::Greater;
            }
            b.version.cmp(&a.version)
        });
        assert_eq!(releases[0].version, "master");
        assert_eq!(releases[1].version, "0.13.0");
        assert_eq!(releases[2].version, "0.12.0");
    }
}
