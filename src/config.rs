use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Config file structures
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct FlagDef {
    pub short: String,
    pub long: String,
    pub description: String,
    pub os: String,
}

#[derive(Debug, Deserialize)]
pub struct LdxConfig {
    #[serde(default)]
    pub flags: HashMap<String, FlagDef>,
}

pub fn load_config() -> Result<LdxConfig> {
    let config_path = std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("."))
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join("config.toml");

    if !config_path.exists() {
        anyhow::bail!(
            "config.toml not found at {}\n\nPlease run install.sh to set up ldx properly.",
            config_path.display()
        );
    }

    let contents = std::fs::read_to_string(&config_path)?;
    let config: LdxConfig = toml::from_str(&contents)?;
    Ok(config)
}

pub fn is_flag_available(flag: &FlagDef) -> bool {
    match flag.os.as_str() {
        "all" => true,
        #[cfg(windows)]
        "windows" => true,
        #[cfg(target_os = "linux")]
        "linux" => true,
        #[cfg(target_os = "macos")]
        "macos" => true,
        _ => false,
    }
}
