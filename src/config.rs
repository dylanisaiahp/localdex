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

pub const DEFAULT_CONFIG: &str = r#"# ldx configuration file
# Edit this file to customise flags and behaviour
#
# os values: "all", "windows", "linux", "macos"

[flags.all-files]
short = "a"
long = "all-files"
description = "Count all files, no filter needed"
os = "all"

[flags.all-drives]
short = "A"
long = "all-drives"
description = "Scan all drives with a per-drive breakdown and total"
os = "windows"

[flags.dir]
short = "d"
long = "dir"
description = "Directory to search (default: current)"
os = "all"

[flags.dirs]
short = "D"
long = "dirs"
description = "Search for directories instead of files"
os = "all"

[flags.extension]
short = "e"
long = "extension"
description = "Search by file extension (e.g. pdf, rs)"
os = "all"

[flags.where]
short = "w"
long = "where"
description = "Print the path of the matched file or directory with a cd hint"
os = "all"

[flags.first]
short = "1"
long = "first"
description = "Stop after the first match"
os = "all"

[flags.help]
short = "h"
long = "help"
description = "Show this help message"
os = "all"

[flags.limit]
short = "L"
long = "limit"
description = "Stop after N matches (e.g. -L 5)"
os = "all"

[flags.open]
short = "o"
long = "open"
description = "Open or launch the matched file"
os = "all"

[flags.quiet]
short = "q"
long = "quiet"
description = "Suppress per-file output; still prints summary count"
os = "all"

[flags.case-sensitive]
short = "s"
long = "case-sensitive"
description = "Case-sensitive search"
os = "all"

[flags.stats]
short = "S"
long = "stats"
description = "Show scan statistics"
os = "all"

[flags.threads]
short = "t"
long = "threads"
description = "Number of threads to use (default: all available)"
os = "all"

[flags.verbose]
short = "v"
long = "verbose"
description = "Show detailed scan breakdown (files + dirs separately)"
os = "all"
"#;

pub fn load_config() -> Result<LdxConfig> {
    let config_path = std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("."))
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join("config.toml");

    if !config_path.exists() {
        std::fs::write(&config_path, DEFAULT_CONFIG)?;
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
