use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::{cache, symlink};

/// Print locally cached versions.
pub fn list_local() -> Result<()> {
    let versions = cache::cached_versions()?;
    let active = symlink::active_version();

    if versions.is_empty() {
        println!("No cached Zig versions found.");
        println!("Run `z install <version>` to install one.");
        return Ok(());
    }

    println!("Cached Zig versions:");
    for v in &versions {
        let marker = if Some(v) == active.as_ref() {
            " (active)"
        } else {
            ""
        };
        println!("  {v}{marker}");
    }

    Ok(())
}

/// Interactive version picker using arrow keys.
pub fn interactive_picker() -> Result<()> {
    let versions = cache::cached_versions()?;

    if versions.is_empty() {
        println!("No cached Zig versions found.");
        println!("Run `z install <version>` or `z ls-remote` to get started.");
        return Ok(());
    }

    let active = symlink::active_version();
    let items: Vec<String> = versions
        .iter()
        .map(|v| {
            if Some(v) == active.as_ref() {
                format!("{v} *")
            } else {
                v.clone()
            }
        })
        .collect();

    let default = versions
        .iter()
        .position(|v| Some(v) == active.as_ref())
        .unwrap_or(0);

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a Zig version (arrow keys, Enter to install, q to quit)")
        .default(default)
        .items(&items)
        .interact_opt()?
        .unwrap_or(usize::MAX);

    if selection == usize::MAX {
        return Ok(());
    }

    let chosen = &versions[selection];
    crate::install::install(chosen)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    /// Build a formatted list string the same way `list_local` does.
    fn format_versions(versions: &[String], active: Option<&str>) -> Vec<String> {
        versions
            .iter()
            .map(|v| {
                let marker = if Some(v.as_str()) == active {
                    " (active)"
                } else {
                    ""
                };
                format!("  {v}{marker}")
            })
            .collect()
    }

    /// Build the interactive-picker item list the same way `interactive_picker` does.
    fn format_items(versions: &[String], active: Option<&str>) -> Vec<String> {
        versions
            .iter()
            .map(|v| {
                if Some(v.as_str()) == active {
                    format!("{v} *")
                } else {
                    v.clone()
                }
            })
            .collect()
    }

    fn with_temp_env<F: FnOnce()>(cache_dir: &std::path::Path, prefix_dir: &std::path::Path, f: F) {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var("Z_CACHE_DIR", cache_dir);
        std::env::set_var("Z_PREFIX", prefix_dir);
        f();
        std::env::remove_var("Z_CACHE_DIR");
        std::env::remove_var("Z_PREFIX");
    }

    #[test]
    fn format_no_active() {
        let versions = ["0.13.0".to_string(), "0.12.0".to_string()];
        let lines = format_versions(&versions, None);
        assert_eq!(lines, vec!["  0.13.0", "  0.12.0"]);
    }

    #[test]
    fn format_with_active() {
        let versions = ["0.13.0".to_string(), "0.12.0".to_string()];
        let lines = format_versions(&versions, Some("0.13.0"));
        assert_eq!(lines, vec!["  0.13.0 (active)", "  0.12.0"]);
    }

    #[test]
    fn format_items_no_active() {
        let versions = ["0.13.0".to_string(), "0.12.0".to_string()];
        let items = format_items(&versions, None);
        assert_eq!(items, vec!["0.13.0", "0.12.0"]);
    }

    #[test]
    fn format_items_marks_active() {
        let versions = ["0.13.0".to_string(), "0.12.0".to_string()];
        let items = format_items(&versions, Some("0.13.0"));
        assert_eq!(items[0], "0.13.0 *");
        assert_eq!(items[1], "0.12.0");
    }

    #[test]
    fn default_index_for_active_version() {
        let versions = [
            "0.13.0".to_string(),
            "0.12.0".to_string(),
            "0.11.0".to_string(),
        ];
        let active = Some("0.12.0".to_string());
        let idx = versions
            .iter()
            .position(|v| Some(v) == active.as_ref())
            .unwrap_or(0);
        assert_eq!(idx, 1);
    }

    #[test]
    fn default_index_falls_back_to_zero_when_no_active() {
        let versions = ["0.13.0".to_string()];
        let idx = versions
            .iter()
            .position(|v| Some(v) == None::<String>.as_ref())
            .unwrap_or(0);
        assert_eq!(idx, 0);
    }

    #[test]
    fn list_local_works_with_empty_cache() {
        let cache_dir = tempfile::tempdir().expect("tempdir");
        let prefix_dir = tempfile::tempdir().expect("tempdir");
        fs::remove_dir_all(cache_dir.path()).unwrap();
        with_temp_env(cache_dir.path(), prefix_dir.path(), || {
            assert!(super::list_local().is_ok());
        });
    }

    #[test]
    fn list_local_works_with_versions() {
        let cache_dir = tempfile::tempdir().expect("tempdir");
        let prefix_dir = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(cache_dir.path().join("0.13.0")).unwrap();
        with_temp_env(cache_dir.path(), prefix_dir.path(), || {
            assert!(super::list_local().is_ok());
        });
    }
}
