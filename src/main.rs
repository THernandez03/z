#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

mod arch;
mod cache;
mod install;
mod list;
mod releases;
mod symlink;

use anyhow::Result;
use clap::{Parser, Subcommand};

/// z — Interactively manage your Zig versions
#[derive(Parser)]
#[command(name = "z", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Version to install (e.g. 0.13.0, master)
    version: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a Zig version
    Install {
        /// Version to install (e.g. 0.13.0, master, lts, latest, canary, next)
        version: String,
    },
    /// List locally cached versions
    Ls,
    /// List remote versions available for download
    LsRemote,
    /// Remove one or more cached versions
    Rm {
        /// Versions to remove
        versions: Vec<String>,
    },
    /// Remove all cached versions except the active one
    Prune,
    /// Show path to a cached Zig binary
    Which {
        /// Version to look up
        version: String,
    },
    /// Run a specific cached Zig version
    Run {
        /// Version to run
        version: String,
        /// Arguments to pass to zig
        args: Vec<String>,
    },
    /// Download a version into cache without activating it
    Download {
        /// Version to download
        version: String,
    },
    /// Show diagnostic information
    Doctor,
    /// Uninstall the active Zig (does not remove cache)
    Uninstall,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => {
            // If a bare version string was given, treat as install
            if let Some(version) = cli.version {
                install::install(&version)?;
            } else {
                // Interactive picker
                list::interactive_picker()?;
            }
        }
        Some(Commands::Install { version }) => install::install(&version)?,
        Some(Commands::Ls) => list::list_local()?,
        Some(Commands::LsRemote) => releases::list_remote()?,
        Some(Commands::Rm { versions }) => {
            for v in &versions {
                cache::remove(v)?;
            }
        }
        Some(Commands::Prune) => cache::prune()?,
        Some(Commands::Which { version }) => {
            let path = cache::which(&version)?;
            println!("{}", path.display());
        }
        Some(Commands::Run { version, args }) => install::run(&version, &args)?,
        Some(Commands::Download { version }) => install::download_only(&version)?,
        Some(Commands::Doctor) => diagnostics::doctor(),
        Some(Commands::Uninstall) => symlink::uninstall(),
    }

    Ok(())
}

mod diagnostics {
    use crate::{cache, symlink};

    pub fn doctor() {
        println!("z — Zig version manager diagnostics");
        println!();

        let prefix = symlink::prefix();
        println!("  install prefix : {}", prefix.display());

        let cache_dir = cache::cache_dir();
        println!("  cache dir      : {}", cache_dir.display());

        let active = symlink::active_version();
        match active {
            Some(v) => println!("  active version : {v}"),
            None => println!("  active version : (none)"),
        }
    }
}
