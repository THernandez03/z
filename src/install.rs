use anyhow::{Context, Result};
use std::fs;
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;
use std::process::Command;

use console::style;

use crate::{cache, releases, symlink};

/// Install a Zig version and activate it.
pub fn install(version_str: &str) -> Result<()> {
    let release = releases::resolve(version_str)?;
    let version = &release.version;
    let from = symlink::active_version();

    if from.as_deref() == Some(version.as_str()) {
        println!(
            "{} Zig {} is already the active version.",
            style("✓").green().bold(),
            style(version).cyan().bold(),
        );
        return Ok(());
    }

    if cache::is_cached(version) {
        println!(
            "{} Zig {} is already cached.",
            style("◆").dim(),
            style(version).cyan(),
        );
    } else {
        println!(
            "{} Downloading Zig {}...",
            style("⬇").cyan(),
            style(version).cyan().bold(),
        );
        download_version(&release.tarball_url, version)?;
    }

    match &from {
        Some(f) => println!(
            "{} Activating Zig {} → {}...",
            style("◆").magenta(),
            style(f).cyan(),
            style(version).cyan().bold(),
        ),
        None => println!(
            "{} Activating Zig {}...",
            style("◆").magenta(),
            style(version).cyan().bold(),
        ),
    }
    symlink::activate(version)?;
    println!(
        "{} Installed Zig {} successfully.",
        style("✓").green().bold(),
        style(version).cyan().bold(),
    );
    Ok(())
}

/// Download a version into cache without activating it.
pub fn download_only(version_str: &str) -> Result<()> {
    let release = releases::resolve(version_str)?;
    let version = &release.version;
    if cache::is_cached(version) {
        println!("Version {version} is already cached.");
        return Ok(());
    }
    println!("Downloading Zig {version}...");
    download_version(&release.tarball_url, version)
}

/// Run a cached Zig version with given arguments.
pub fn run(version_str: &str, args: &[String]) -> Result<()> {
    let release = releases::resolve(version_str)?;
    let version = &release.version;

    if !cache::is_cached(version) {
        println!("Version {version} is not cached. Downloading...");
        download_version(&release.tarball_url, version)?;
    }

    let binary = cache::zig_binary(version);
    let status = Command::new(&binary)
        .args(args)
        .status()
        .with_context(|| format!("Failed to run zig {version}"))?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

fn download_version(url: &str, version: &str) -> Result<()> {
    let dest_dir = cache::version_dir(version);
    fs::create_dir_all(&dest_dir).context("Failed to create cache directory")?;

    // Determine archive type
    let is_zip = std::path::Path::new(url)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("zip"));
    let is_xz = url.ends_with(".tar.xz");

    // Download to a temp file
    let tmp_path = dest_dir.with_extension(if is_zip { "zip" } else { "tar.xz" });

    {
        let client = reqwest::blocking::Client::new();
        let mut resp = client
            .get(url)
            .header("User-Agent", "z-zig-version-manager")
            .send()
            .context("HTTP request failed")?;

        let total = resp.content_length().unwrap_or(0);
        let file = fs::File::create(&tmp_path).context("Failed to create temp file")?;
        let mut writer = BufWriter::new(file);

        let mut downloaded = 0u64;
        let mut buf = vec![0u8; 65536];
        loop {
            let n = resp.read(&mut buf)?;
            if n == 0 {
                break;
            }
            writer.write_all(&buf[..n])?;
            downloaded += n as u64;
            if let Some(pct) = (downloaded * 100).checked_div(total) {
                print!("\r  {downloaded}/{total} bytes ({pct}%)");
                io::stdout().flush()?;
            }
        }
        println!();
    }

    // Extract archive
    if is_zip {
        extract_zip(&tmp_path, &dest_dir)?;
    } else if is_xz {
        extract_tar_xz(&tmp_path, &dest_dir)?;
    }

    fs::remove_file(&tmp_path).ok();

    // Zig archives contain a single top-level directory; flatten it.
    flatten_single_dir(&dest_dir)?;

    Ok(())
}

fn extract_tar_xz(archive: &Path, dest: &Path) -> Result<()> {
    let file = fs::File::open(archive).context("Failed to open tar.xz")?;
    let xz = xz2::read::XzDecoder::new(file);
    let mut tar = tar::Archive::new(xz);
    tar.unpack(dest).context("Failed to extract tar.xz")?;
    Ok(())
}

fn extract_zip(archive: &Path, dest: &Path) -> Result<()> {
    let file = fs::File::open(archive).context("Failed to open zip")?;
    let mut zip = zip::ZipArchive::new(file).context("Failed to read zip")?;
    zip.extract(dest).context("Failed to extract zip")?;
    Ok(())
}

/// If `dir` contains exactly one subdirectory, move its contents up one level.
fn flatten_single_dir(dir: &Path) -> Result<()> {
    let entries: Vec<_> = fs::read_dir(dir)?.collect::<std::io::Result<_>>()?;
    if entries.len() == 1 && entries[0].path().is_dir() {
        let inner = entries[0].path();
        for entry in fs::read_dir(&inner)? {
            let entry = entry?;
            let dest = dir.join(entry.file_name());
            fs::rename(entry.path(), dest).ok();
        }
        fs::remove_dir_all(&inner).ok();
    }
    Ok(())
}
