use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Default config — source of truth for --sync and --reset
// ---------------------------------------------------------------------------

pub const DEFAULT_CONFIG: &str = include_str!("../default_config.toml");

// ---------------------------------------------------------------------------
// Config structures
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct FlagDef {
    pub short: String,
    pub long: String,
    pub description: String,
    pub os: String,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LdxConfig {
    #[serde(default)]
    pub flags: HashMap<String, FlagDef>,
    #[serde(default)]
    pub custom: HashMap<String, FlagDef>,
    #[serde(default)]
    pub aliases: HashMap<String, String>,
}

// ---------------------------------------------------------------------------
// Config path
// ---------------------------------------------------------------------------

pub fn config_path() -> PathBuf {
    std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("."))
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join("config.toml")
}

// ---------------------------------------------------------------------------
// Load config
// ---------------------------------------------------------------------------

pub fn load_config() -> Result<LdxConfig> {
    let path = config_path();

    if !path.exists() {
        anyhow::bail!(
            "config.toml not found at {}\n\nPlease run install.sh to set up ldx properly.",
            path.display()
        );
    }

    let contents = std::fs::read_to_string(&path)?;
    let config: LdxConfig = toml::from_str(&contents)?;
    Ok(config)
}

// ---------------------------------------------------------------------------
// Flag availability check
// ---------------------------------------------------------------------------

pub fn is_flag_available(flag: &FlagDef) -> bool {
    flag.os == "all"
        || (cfg!(windows) && flag.os == "windows")
        || (cfg!(target_os = "linux") && flag.os == "linux")
        || (cfg!(target_os = "macos") && flag.os == "macos")
}

// Re-export config management functions from config_check
pub use crate::config_check::{check_config, reset_config, sync_config};

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_flag(os: &str) -> FlagDef {
        FlagDef {
            short: "q".into(), long: "quiet".into(),
            description: "Quiet".into(), os: os.into(),
            action: None, target: None, value: None,
        }
    }

    #[test]
    fn flag_available_for_all_os() {
        assert!(is_flag_available(&make_flag("all")));
    }

    #[test]
    fn flag_available_for_linux() {
        #[cfg(target_os = "linux")]
        assert!(is_flag_available(&make_flag("linux")));
    }

    #[test]
    fn flag_not_available_for_windows_on_linux() {
        #[cfg(target_os = "linux")]
        assert!(!is_flag_available(&make_flag("windows")));
    }

    #[test]
    fn flag_not_available_for_macos_on_linux() {
        #[cfg(target_os = "linux")]
        assert!(!is_flag_available(&make_flag("macos")));
    }

    #[test]
    fn default_config_is_valid_toml() {
        let result = toml::from_str::<LdxConfig>(DEFAULT_CONFIG);
        assert!(result.is_ok(), "DEFAULT_CONFIG failed to parse: {:?}", result.err());
    }
}
