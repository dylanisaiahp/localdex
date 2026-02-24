use anyhow::{Result, bail};
use std::path::PathBuf;

use crate::config::LdxConfig;

// ---------------------------------------------------------------------------
// Parsed flags
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct ParsedFlags {
    pub pattern: Option<String>,
    pub dir: PathBuf,
    pub extension: Option<String>,
    pub threads: usize,
    pub quiet: bool,
    pub stats: bool,
    pub all: bool,
    pub verbose: bool,
    pub open: bool,
    pub dirs_only: bool,
    pub where_mode: bool,
    pub all_drives: bool,
    pub case_sensitive: bool,
    pub limit: Option<usize>,
    pub exclude: Vec<String>,
    pub show_help: bool,
    pub show_version: bool,
    pub show_config: bool,
    pub edit_config: bool,
    pub check_config: bool,
    pub sync_config: bool,
    pub reset_config: bool,
    pub warn: bool,
}

// ---------------------------------------------------------------------------
// Expand aliases before parsing
// ---------------------------------------------------------------------------

pub fn expand_aliases(args: Vec<String>, config: &LdxConfig) -> Vec<String> {
    args.into_iter()
        .flat_map(|arg| {
            if let Some(expansion) = config.aliases.get(&arg) {
                expansion
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect()
            } else {
                vec![arg]
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Resolve custom flags into their target args
// ---------------------------------------------------------------------------

pub fn resolve_custom(args: Vec<String>, config: &LdxConfig) -> Vec<String> {
    let mut resolved = Vec::new();

    for arg in &args {
        let mut matched = false;

        for custom in config.custom.values() {
            let short = format!("-{}", custom.short);
            let long = format!("--{}", custom.long);

            if *arg == short || *arg == long {
                if let (Some(action), Some(target)) = (&custom.action, &custom.target) {
                    let find_flag = || {
                        config
                            .flags
                            .values()
                            .find(|f| f.target.as_deref() == Some(target.as_str()))
                            .map(|f| format!("-{}", f.short))
                            .unwrap_or_default()
                    };

                    match action.as_str() {
                        "set_value" => {
                            if let Some(value) = &custom.value {
                                let flag = find_flag();
                                if !flag.is_empty() {
                                    resolved.push(flag);
                                    resolved.push(value.clone());
                                }
                            }
                        }
                        "set_boolean" => {
                            let flag = find_flag();
                            if !flag.is_empty() {
                                resolved.push(flag);
                            }
                        }
                        _ => resolved.push(arg.clone()),
                    }
                }
                matched = true;
                break;
            }
        }

        if !matched {
            resolved.push(arg.clone());
        }
    }

    resolved
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn get_flag_names(config: &LdxConfig, target: &str) -> (String, String) {
    config
        .flags
        .values()
        .find(|f| f.target.as_deref() == Some(target))
        .map(|f| (format!("-{}", f.short), format!("--{}", f.long)))
        .unwrap_or_default()
}

fn flag_matches(arg: &str, short: &str, long: &str) -> bool {
    arg == short || arg == long
}

// ---------------------------------------------------------------------------
// Management flags (early exit)
// ---------------------------------------------------------------------------

fn parse_management(raw: &[String], config: &LdxConfig) -> Option<ParsedFlags> {
    let (help_s, help_l) = get_flag_names(config, "help");
    let max_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let show_help = raw
        .iter()
        .any(|a| flag_matches(a, &help_s, &help_l) || a == "-h" || a == "--help");
    let show_version = raw.iter().any(|a| a == "--version");
    let show_config = raw.iter().any(|a| a == "--config");
    let edit_config = raw.iter().any(|a| a == "--edit");
    let check_config = raw.iter().any(|a| a == "--check");
    let sync_config = raw.iter().any(|a| a == "--sync");
    let reset_config = raw.iter().any(|a| a == "--reset");

    if show_help
        || show_version
        || show_config
        || edit_config
        || check_config
        || sync_config
        || reset_config
    {
        return Some(ParsedFlags {
            show_help,
            show_version,
            show_config,
            edit_config,
            check_config,
            sync_config,
            reset_config,
            dir: ".".into(),
            threads: max_threads,
            ..Default::default()
        });
    }

    None
}

// ---------------------------------------------------------------------------
// Value flags
// ---------------------------------------------------------------------------

struct ValueFlags {
    extension: Option<String>,
    dir: PathBuf,
    threads: usize,
    limit: Option<usize>,
    exclude: Vec<String>,
}

fn parse_value_flags(raw: &[String], config: &LdxConfig) -> ValueFlags {
    let max_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let (ext_s, ext_l) = get_flag_names(config, "extension");
    let (dir_s, dir_l) = get_flag_names(config, "dir");
    let (threads_s, threads_l) = get_flag_names(config, "threads");
    let (limit_s, limit_l) = get_flag_names(config, "limit");

    let extension = raw
        .iter()
        .position(|a| flag_matches(a, &ext_s, &ext_l))
        .and_then(|i| raw.get(i + 1))
        .map(|s| s.trim_start_matches('.').to_lowercase());

    let dir = raw
        .iter()
        .position(|a| flag_matches(a, &dir_s, &dir_l))
        .and_then(|i| raw.get(i + 1))
        .map(PathBuf::from)
        .unwrap_or_else(|| ".".into());

    let threads = raw.iter().position(|a| flag_matches(a, &threads_s, &threads_l))
        .and_then(|i| raw.get(i + 1))
        .and_then(|v| v.parse().ok())
        .map(|n: usize| {
            if n > max_threads {
                eprintln!("Warning: {} threads requested but only {} logical cores available. Capping at {}.", n, max_threads, max_threads);
                max_threads
            } else {
                n
            }
        })
        .unwrap_or(max_threads);

    let limit = raw
        .iter()
        .position(|a| flag_matches(a, &limit_s, &limit_l))
        .and_then(|i| raw.get(i + 1))
        .and_then(|v| v.parse().ok());

    let exclude = raw
        .iter()
        .position(|a| a == "--exclude")
        .and_then(|i| raw.get(i + 1))
        .map(|s| s.split(',').map(|p| p.trim().to_string()).collect())
        .unwrap_or_default();

    ValueFlags {
        extension,
        dir,
        threads,
        limit,
        exclude,
    }
}

// ---------------------------------------------------------------------------
// Boolean flags
// ---------------------------------------------------------------------------

struct BoolFlags {
    quiet: bool,
    stats: bool,
    all: bool,
    verbose: bool,
    first: bool,
    open: bool,
    dirs_only: bool,
    where_mode: bool,
    case_sensitive: bool,
    all_drives: bool,
    warn: bool,
}

fn parse_bool_flags(raw: &[String], config: &LdxConfig) -> BoolFlags {
    let (quiet_s, quiet_l) = get_flag_names(config, "quiet");
    let (stats_s, stats_l) = get_flag_names(config, "stats");
    let (all_s, all_l) = get_flag_names(config, "all");
    let (verbose_s, verbose_l) = get_flag_names(config, "verbose");
    let (first_s, first_l) = get_flag_names(config, "first");
    let (open_s, open_l) = get_flag_names(config, "open");
    let (dirs_s, dirs_l) = get_flag_names(config, "dirs_only");
    let (where_s, where_l) = get_flag_names(config, "where_mode");
    let (cs_s, cs_l) = get_flag_names(config, "case_sensitive");
    let (drives_s, drives_l) = get_flag_names(config, "all_drives");

    #[cfg(windows)]
    let all_drives = raw.iter().any(|a| flag_matches(a, &drives_s, &drives_l));
    #[cfg(not(windows))]
    let all_drives = {
        let _ = (&drives_s, &drives_l);
        false
    };

    BoolFlags {
        quiet: raw.iter().any(|a| flag_matches(a, &quiet_s, &quiet_l)),
        stats: raw.iter().any(|a| flag_matches(a, &stats_s, &stats_l)),
        all: raw.iter().any(|a| flag_matches(a, &all_s, &all_l)),
        verbose: raw.iter().any(|a| flag_matches(a, &verbose_s, &verbose_l)),
        first: raw.iter().any(|a| flag_matches(a, &first_s, &first_l)),
        open: raw.iter().any(|a| flag_matches(a, &open_s, &open_l)),
        dirs_only: raw.iter().any(|a| flag_matches(a, &dirs_s, &dirs_l)),
        where_mode: raw.iter().any(|a| flag_matches(a, &where_s, &where_l)),
        case_sensitive: raw.iter().any(|a| flag_matches(a, &cs_s, &cs_l)),
        all_drives,
        warn: raw.iter().any(|a| a == "--warn" || a == "-W"),
    }
}

// ---------------------------------------------------------------------------
// Pattern extraction + unknown flag validation
// ---------------------------------------------------------------------------

fn parse_pattern(raw: &[String], config: &LdxConfig) -> Result<Option<String>> {
    let value_flag_names: Vec<String> = {
        let (ext_s, ext_l) = get_flag_names(config, "extension");
        let (dir_s, dir_l) = get_flag_names(config, "dir");
        let (threads_s, threads_l) = get_flag_names(config, "threads");
        let (limit_s, limit_l) = get_flag_names(config, "limit");
        vec![
            ext_s,
            ext_l,
            dir_s,
            dir_l,
            threads_s,
            threads_l,
            limit_s,
            limit_l,
            "--exclude".into(),
        ]
    };

    let known_flags: Vec<String> = config
        .flags
        .values()
        .flat_map(|f| vec![format!("-{}", f.short), format!("--{}", f.long)])
        .chain(
            [
                "--version",
                "--help",
                "-h",
                "--config",
                "--edit",
                "--check",
                "--sync",
                "--reset",
                "--exclude",
                "--warn",
            ]
            .iter()
            .map(|s| s.to_string()),
        )
        .collect();

    let mut pattern: Option<String> = None;
    let mut skip_next = false;

    for arg in raw {
        if skip_next {
            skip_next = false;
            continue;
        }
        if value_flag_names.contains(arg) {
            skip_next = true;
            continue;
        }
        if arg.starts_with('-') {
            if !known_flags.contains(arg) {
                bail!("Unknown flag: {:?}. Run with --help for usage.", arg);
            }
        } else if pattern.is_none() {
            pattern = Some(arg.clone());
        }
    }

    Ok(pattern)
}

// ---------------------------------------------------------------------------
// Validate flag combinations
// ---------------------------------------------------------------------------

pub fn validate_combos(
    pattern: &Option<String>,
    extension: &Option<String>,
    first: bool,
    limit: Option<usize>,
    all: bool,
    open: bool,
    dirs_only: bool,
) -> Result<()> {
    if first && limit.is_some() {
        bail!("-1/--first and -L/--limit cannot be used together.");
    }
    if all && (pattern.is_some() || extension.is_some()) {
        bail!("-a/--all-files cannot be combined with a pattern or -e/--extension.");
    }
    if open && all {
        bail!("-o/--open cannot be combined with -a/--all-files.");
    }
    if dirs_only && all {
        bail!("-D/--dirs cannot be combined with -a/--all-files.");
    }
    if dirs_only && extension.is_some() {
        bail!("-D/--dirs cannot be combined with -e/--extension.");
    }
    if pattern.is_some() && extension.is_some() {
        bail!("Cannot use both a pattern and -e/--extension at the same time.");
    }
    if !all && pattern.is_none() && extension.is_none() {
        bail!(
            "Either a pattern, -e/--extension, or -a/--all-files is required. Run with --help for usage."
        );
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// parse_args — orchestration
// ---------------------------------------------------------------------------

pub fn parse_args(config: &LdxConfig) -> Result<ParsedFlags> {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let raw = expand_aliases(raw, config);
    let raw = resolve_custom(raw, config);

    if let Some(flags) = parse_management(&raw, config) {
        return Ok(flags);
    }

    let v = parse_value_flags(&raw, config);
    let b = parse_bool_flags(&raw, config);
    let pattern = parse_pattern(&raw, config)?;

    validate_combos(
        &pattern,
        &v.extension,
        b.first,
        v.limit,
        b.all,
        b.open,
        b.dirs_only,
    )?;

    let limit = if b.first || b.where_mode {
        Some(1)
    } else {
        v.limit
    };

    Ok(ParsedFlags {
        pattern,
        dir: v.dir,
        extension: v.extension,
        threads: v.threads,
        quiet: b.quiet,
        stats: b.stats,
        all: b.all,
        verbose: b.verbose,
        open: b.open,
        dirs_only: b.dirs_only,
        where_mode: b.where_mode,
        all_drives: b.all_drives,
        case_sensitive: b.case_sensitive,
        limit,
        exclude: v.exclude,
        show_help: false,
        show_version: false,
        show_config: false,
        edit_config: false,
        check_config: false,
        sync_config: false,
        reset_config: false,
        warn: b.warn,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FlagDef;
    use std::collections::HashMap;

    fn make_config() -> LdxConfig {
        let mut flags = HashMap::new();
        flags.insert(
            "extension".into(),
            FlagDef {
                short: "e".into(),
                long: "extension".into(),
                description: "Extension".into(),
                os: "all".into(),
                action: None,
                target: Some("extension".into()),
                value: None,
            },
        );
        flags.insert(
            "quiet".into(),
            FlagDef {
                short: "q".into(),
                long: "quiet".into(),
                description: "Quiet".into(),
                os: "all".into(),
                action: None,
                target: Some("quiet".into()),
                value: None,
            },
        );
        flags.insert(
            "stats".into(),
            FlagDef {
                short: "S".into(),
                long: "stats".into(),
                description: "Stats".into(),
                os: "all".into(),
                action: None,
                target: Some("stats".into()),
                value: None,
            },
        );
        LdxConfig {
            flags,
            custom: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    #[test]
    fn expand_aliases_replaces_known_alias() {
        let mut config = make_config();
        config.aliases.insert("ct".into(), "-a -S -q".into());
        let expanded = expand_aliases(vec!["ct".to_string()], &config);
        assert_eq!(expanded, vec!["-a", "-S", "-q"]);
    }

    #[test]
    fn expand_aliases_leaves_unknown_args_untouched() {
        let config = make_config();
        let expanded = expand_aliases(vec!["invoice".to_string(), "-q".to_string()], &config);
        assert_eq!(expanded, vec!["invoice", "-q"]);
    }

    #[test]
    fn expand_aliases_handles_empty_args() {
        let config = make_config();
        assert!(expand_aliases(vec![], &config).is_empty());
    }

    #[test]
    fn expand_aliases_expands_multiple_aliases() {
        let mut config = make_config();
        config.aliases.insert("ct".into(), "-a -S".into());
        config.aliases.insert("qq".into(), "-q".into());
        let expanded = expand_aliases(vec!["ct".to_string(), "qq".to_string()], &config);
        assert_eq!(expanded, vec!["-a", "-S", "-q"]);
    }

    #[test]
    fn resolve_custom_expands_set_value_flag() {
        let mut config = make_config();
        config.custom.insert(
            "rust".into(),
            FlagDef {
                short: "R".into(),
                long: "rust".into(),
                description: "Rust files".into(),
                os: "all".into(),
                action: Some("set_value".into()),
                target: Some("extension".into()),
                value: Some("rs".into()),
            },
        );
        let resolved = resolve_custom(vec!["-R".to_string()], &config);
        assert_eq!(resolved, vec!["-e", "rs"]);
    }

    #[test]
    fn resolve_custom_leaves_unknown_args_untouched() {
        let config = make_config();
        let resolved = resolve_custom(vec!["invoice".to_string(), "-q".to_string()], &config);
        assert_eq!(resolved, vec!["invoice", "-q"]);
    }

    #[test]
    fn validate_rejects_first_and_limit() {
        assert!(validate_combos(&None, &None, true, Some(5), false, false, false).is_err());
    }

    #[test]
    fn validate_rejects_all_with_pattern() {
        assert!(
            validate_combos(
                &Some("invoice".into()),
                &None,
                false,
                None,
                true,
                false,
                false
            )
            .is_err()
        );
    }

    #[test]
    fn validate_rejects_all_with_extension() {
        assert!(
            validate_combos(&None, &Some("rs".into()), false, None, true, false, false).is_err()
        );
    }

    #[test]
    fn validate_rejects_open_with_all() {
        assert!(validate_combos(&None, &None, false, None, true, true, false).is_err());
    }

    #[test]
    fn validate_rejects_dirs_with_all() {
        assert!(validate_combos(&None, &None, false, None, true, false, true).is_err());
    }

    #[test]
    fn validate_rejects_dirs_with_extension() {
        assert!(
            validate_combos(&None, &Some("rs".into()), false, None, false, false, true).is_err()
        );
    }

    #[test]
    fn validate_rejects_pattern_and_extension() {
        assert!(
            validate_combos(
                &Some("invoice".into()),
                &Some("rs".into()),
                false,
                None,
                false,
                false,
                false
            )
            .is_err()
        );
    }

    #[test]
    fn validate_rejects_no_search_criteria() {
        assert!(validate_combos(&None, &None, false, None, false, false, false).is_err());
    }

    #[test]
    fn validate_accepts_pattern_alone() {
        assert!(
            validate_combos(
                &Some("invoice".into()),
                &None,
                false,
                None,
                false,
                false,
                false
            )
            .is_ok()
        );
    }

    #[test]
    fn validate_accepts_all_alone() {
        assert!(validate_combos(&None, &None, false, None, true, false, false).is_ok());
    }

    #[test]
    fn validate_accepts_extension_alone() {
        assert!(
            validate_combos(&None, &Some("rs".into()), false, None, false, false, false).is_ok()
        );
    }
}
