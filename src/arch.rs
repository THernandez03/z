/// Returns the Zig architecture-OS string for the current platform,
/// e.g. "x86_64-linux", "aarch64-macos", "x86_64-windows".
pub fn target() -> &'static str {
    // Architecture
    #[cfg(target_arch = "x86_64")]
    let arch = "x86_64";
    #[cfg(target_arch = "aarch64")]
    let arch = "aarch64";
    #[cfg(target_arch = "arm")]
    let arch = "arm";
    #[cfg(target_arch = "riscv64")]
    let arch = "riscv64";
    #[cfg(not(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "riscv64"
    )))]
    let arch = "x86_64"; // fallback

    // OS
    #[cfg(target_os = "linux")]
    let os = "linux";
    #[cfg(target_os = "macos")]
    let os = "macos";
    #[cfg(target_os = "windows")]
    let os = "windows";
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    let os = "linux"; // fallback

    // Combine at compile time via a match on a const tuple would be complex;
    // instead we build a &'static str via a helper.
    target_str(arch, os)
}

fn target_str(arch: &str, os: &str) -> &'static str {
    match (arch, os) {
        ("aarch64", "linux") => "aarch64-linux",
        ("arm", "linux") => "arm-linux",
        ("riscv64", "linux") => "riscv64-linux",
        ("x86_64", "macos") => "x86_64-macos",
        ("aarch64", "macos") => "aarch64-macos",
        ("x86_64", "windows") => "x86_64-windows",
        ("aarch64", "windows") => "aarch64-windows",
        // "x86_64-linux" and any unknown combination
        _ => "x86_64-linux",
    }
}

/// Returns the file extension used by Zig tarballs for the current platform.
pub const fn archive_ext() -> &'static str {
    #[cfg(target_os = "windows")]
    return "zip";
    #[cfg(not(target_os = "windows"))]
    return "tar.xz";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_is_nonempty() {
        assert!(!target().is_empty());
    }

    #[test]
    fn target_contains_dash() {
        // All valid targets are "<arch>-<os>"
        assert!(target().contains('-'));
    }

    #[test]
    fn target_str_known_combinations() {
        // x86_64-linux falls through to the wildcard arm
        assert_eq!(target_str("x86_64", "linux"), "x86_64-linux");
        assert_eq!(target_str("aarch64", "linux"), "aarch64-linux");
        assert_eq!(target_str("arm", "linux"), "arm-linux");
        assert_eq!(target_str("riscv64", "linux"), "riscv64-linux");
        assert_eq!(target_str("x86_64", "macos"), "x86_64-macos");
        assert_eq!(target_str("aarch64", "macos"), "aarch64-macos");
        assert_eq!(target_str("x86_64", "windows"), "x86_64-windows");
        assert_eq!(target_str("aarch64", "windows"), "aarch64-windows");
    }

    #[test]
    fn target_str_unknown_falls_back() {
        assert_eq!(target_str("mips", "freebsd"), "x86_64-linux");
        assert_eq!(target_str("", ""), "x86_64-linux");
    }

    #[test]
    fn archive_ext_is_nonempty() {
        assert!(!archive_ext().is_empty());
    }

    #[test]
    fn archive_ext_is_known_value() {
        let ext = archive_ext();
        assert!(ext == "tar.xz" || ext == "zip");
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn archive_ext_unix_is_tar_xz() {
        assert_eq!(archive_ext(), "tar.xz");
    }
}
