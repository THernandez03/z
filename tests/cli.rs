/// Binary-level integration tests for the `z` CLI.
///
/// These tests invoke the compiled binary as a user would, exercising the full
/// path from CLI parsing → command dispatch → output.  All tests are offline:
/// they pre-populate a temporary cache directory with a fake binary so that
/// `fetch` and `which` return immediately without hitting the network.
use std::fs;
use std::path::Path;
use std::process::Command;

fn z() -> Command {
    Command::new(env!("CARGO_BIN_EXE_z"))
}

/// Create a fake cached Zig binary at the path `z` expects.
/// Zig binary lives at `{cache}/{version}/zig`.
fn fake_cache(dir: &Path, version: &str) {
    let vdir = dir.join(version);
    fs::create_dir_all(&vdir).unwrap();
    fs::write(vdir.join("zig"), b"#!/bin/sh\necho fake\n").unwrap();
}

// ── --help / --version ────────────────────────────────────────────────────────

#[test]
fn help_exits_zero() {
    assert!(z().arg("--help").status().unwrap().success());
}

#[test]
fn version_prints_semver() {
    let out = z().arg("--version").output().unwrap();
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        s.split_whitespace()
            .any(|w| w.contains('.') && w.chars().next().is_some_and(|c| c.is_ascii_digit())),
        "expected semver in version output, got: {s:?}"
    );
}

#[test]
fn unknown_flag_exits_nonzero() {
    assert!(!z().arg("--not-a-real-flag").status().unwrap().success());
}

// ── ls ────────────────────────────────────────────────────────────────────────

#[test]
fn ls_empty_cache_reports_none() {
    let cache = tempfile::tempdir().unwrap();
    let prefix = tempfile::tempdir().unwrap();
    let out = z()
        .arg("ls")
        .env("Z_CACHE_DIR", cache.path())
        .env("Z_PREFIX", prefix.path())
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("No cached Zig versions found."),
        "unexpected output: {stdout}"
    );
}

#[test]
fn ls_shows_cached_version() {
    let cache = tempfile::tempdir().unwrap();
    let prefix = tempfile::tempdir().unwrap();
    fake_cache(cache.path(), "0.13.0");
    let out = z()
        .arg("ls")
        .env("Z_CACHE_DIR", cache.path())
        .env("Z_PREFIX", prefix.path())
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("0.13.0"),
        "expected version in output: {stdout}"
    );
}

#[test]
fn ls_marks_active_version() {
    let cache = tempfile::tempdir().unwrap();
    let prefix = tempfile::tempdir().unwrap();
    fake_cache(cache.path(), "0.13.0");
    fs::write(prefix.path().join(".active"), "0.13.0").unwrap();
    let out = z()
        .arg("ls")
        .env("Z_CACHE_DIR", cache.path())
        .env("Z_PREFIX", prefix.path())
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("(active)"),
        "expected (active) marker in output: {stdout}"
    );
}

// ── fetch ─────────────────────────────────────────────────────────────────────

#[test]
fn fetch_skips_download_when_already_cached() {
    let cache = tempfile::tempdir().unwrap();
    let prefix = tempfile::tempdir().unwrap();
    fake_cache(cache.path(), "0.13.0");
    let status = z()
        .args(["fetch", "0.13.0"])
        .env("Z_CACHE_DIR", cache.path())
        .env("Z_PREFIX", prefix.path())
        .status()
        .unwrap();
    assert!(
        status.success(),
        "fetch should succeed without network when version is already cached"
    );
}

// ── which ─────────────────────────────────────────────────────────────────────

#[test]
fn which_prints_binary_path() {
    let cache = tempfile::tempdir().unwrap();
    let prefix = tempfile::tempdir().unwrap();
    fake_cache(cache.path(), "0.13.0");
    let out = z()
        .args(["which", "0.13.0"])
        .env("Z_CACHE_DIR", cache.path())
        .env("Z_PREFIX", prefix.path())
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("0.13.0") && stdout.contains("zig"),
        "expected path containing version and 'zig': {stdout}"
    );
}

#[test]
fn which_fails_when_not_cached() {
    let cache = tempfile::tempdir().unwrap();
    let prefix = tempfile::tempdir().unwrap();
    let status = z()
        .args(["which", "0.13.0"])
        .env("Z_CACHE_DIR", cache.path())
        .env("Z_PREFIX", prefix.path())
        .status()
        .unwrap();
    assert!(
        !status.success(),
        "which should fail when the version is not cached"
    );
}

// ── prune ─────────────────────────────────────────────────────────────────────

#[test]
fn prune_removes_inactive_keeps_active() {
    let cache = tempfile::tempdir().unwrap();
    let prefix = tempfile::tempdir().unwrap();
    fake_cache(cache.path(), "0.12.0");
    fake_cache(cache.path(), "0.13.0");
    fs::write(prefix.path().join(".active"), "0.13.0").unwrap();
    z().arg("prune")
        .env("Z_CACHE_DIR", cache.path())
        .env("Z_PREFIX", prefix.path())
        .status()
        .unwrap();
    assert!(
        !cache.path().join("0.12.0").exists(),
        "inactive should be removed"
    );
    assert!(
        cache.path().join("0.13.0").exists(),
        "active should be kept"
    );
}

#[test]
fn prune_force_removes_all_including_active() {
    let cache = tempfile::tempdir().unwrap();
    let prefix = tempfile::tempdir().unwrap();
    fake_cache(cache.path(), "0.12.0");
    fake_cache(cache.path(), "0.13.0");
    fs::write(prefix.path().join(".active"), "0.13.0").unwrap();
    z().args(["prune", "--force"])
        .env("Z_CACHE_DIR", cache.path())
        .env("Z_PREFIX", prefix.path())
        .status()
        .unwrap();
    assert!(
        !cache.path().join("0.12.0").exists(),
        "inactive should be removed by --force"
    );
    assert!(
        !cache.path().join("0.13.0").exists(),
        "active should be removed by --force"
    );
}
