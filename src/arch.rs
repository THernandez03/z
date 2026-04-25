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
        ("x86_64", "linux") => "x86_64-linux",
        ("aarch64", "linux") => "aarch64-linux",
        ("arm", "linux") => "arm-linux",
        ("riscv64", "linux") => "riscv64-linux",
        ("x86_64", "macos") => "x86_64-macos",
        ("aarch64", "macos") => "aarch64-macos",
        ("x86_64", "windows") => "x86_64-windows",
        ("aarch64", "windows") => "aarch64-windows",
        _ => "x86_64-linux",
    }
}

/// Returns the file extension used by Zig tarballs for the current platform.
pub fn archive_ext() -> &'static str {
    #[cfg(target_os = "windows")]
    return "zip";
    #[cfg(not(target_os = "windows"))]
    return "tar.xz";
}
