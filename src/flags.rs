use anyhow::{Result, bail};
use std::path::PathBuf;

use crate::config::LdxConfig;

// ---------------------------------------------------------------------------
// Parsed flags
// ---------------------------------------------------------------------------

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
}

// ---------------------------------------------------------------------------
// Expand aliases before parsing
// ---------------------------------------------------------------------------

pub fn expand_aliases(args: Vec<String>, config: &LdxConfig) -> Vec<String> {
    let mut expanded = Vec::new();
    for arg in args {
        if let Some(alias_value) = config.aliases.get(&arg) {
            for part in alias_value.split_whitespace() {
                expanded.push(part.to_string());
            }
        } else {
            expanded.push(arg);
        }
    }
    expanded
}

// ---------------------------------------------------------------------------
// Resolve custom flags into their target args
// ---------------------------------------------------------------------------

pub fn resolve_custom(args: Vec<String>, config: &LdxConfig) -> Vec<String> {
    let mut resolved = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        let mut matched = false;

        for custom in config.custom.values() {
            let short = format!("-{}", custom.short);
            let long = format!("--{}", custom.long);

            if *arg == short || *arg == long {
                if let (Some(action), Some(target)) = (&custom.action, &custom.target) {
                    match action.as_str() {
                        "set_value" => {
                            if let Some(value) = &custom.value {
                                let flag = config
                                    .flags
                                    .values()
                                    .find(|f| f.target.as_deref() == Some(target.as_str()))
                                    .map(|f| format!("-{}", f.short))
                                    .unwrap_or_default();
                                if !flag.is_empty() {
                                    resolved.push(flag);
                                    resolved.push(value.clone());
                                }
                            }
                        }
                        "set_boolean" => {
                            let flag = config
                                .flags
                                .values()
                                .find(|f| f.target.as_deref() == Some(target.as_str()))
                                .map(|f| format!("-{}", f.short))
                                .unwrap_or_default();
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
        i += 1;
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
// Parse args
// ---------------------------------------------------------------------------

pub fn parse_args(config: &LdxConfig) -> Result<ParsedFlags> {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let raw = expand_aliases(raw, config);
    let raw = resolve_custom(raw, config);

    let max_threads = num_cpus::get();

    let (help_s, help_l) = get_flag_names(config, "help");
    let (quiet_s, quiet_l) = get_flag_names(config, "quiet");
    let (stats_s, stats_l) = get_flag_names(config, "stats");
    let (all_s, all_l) = get_flag_names(config, "all");
    let (verbose_s, verbose_l) = get_flag_names(config, "verbose");
    let (first_s, first_l) = get_flag_names(config, "first");
    let (open_s, open_l) = get_flag_names(config, "open");
    let (dirs_s, dirs_l) = get_flag_names(config, "dirs_only");
    let (where_s, where_l) = get_flag_names(config, "where_mode");
    let (drives_s, drives_l) = get_flag_names(config, "all_drives");
    let (ext_s, ext_l) = get_flag_names(config, "extension");
    let (dir_s, dir_l) = get_flag_names(config, "dir");
    let (threads_s, threads_l) = get_flag_names(config, "threads");
    let (limit_s, limit_l) = get_flag_names(config, "limit");
    let (cs_s, cs_l) = get_flag_names(config, "case_sensitive");

    // Early-exit management flags
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
        return Ok(ParsedFlags {
            show_help,
            show_version,
            show_config,
            edit_config,
            check_config,
            sync_config,
            reset_config,
            pattern: None,
            dir: ".".into(),
            extension: None,
            threads: max_threads,
            quiet: false,
            stats: false,
            all: false,
            verbose: false,
            open: false,
            dirs_only: false,
            where_mode: false,
            all_drives: false,
            case_sensitive: false,
            limit: None,
            exclude: Vec::new(),
        });
    }

    // Boolean flags
    let quiet = raw.iter().any(|a| flag_matches(a, &quiet_s, &quiet_l));
    let stats = raw.iter().any(|a| flag_matches(a, &stats_s, &stats_l));
    let all = raw.iter().any(|a| flag_matches(a, &all_s, &all_l));
    let verbose = raw.iter().any(|a| flag_matches(a, &verbose_s, &verbose_l));
    let first = raw.iter().any(|a| flag_matches(a, &first_s, &first_l));
    let open = raw.iter().any(|a| flag_matches(a, &open_s, &open_l));
    let dirs_only = raw.iter().any(|a| flag_matches(a, &dirs_s, &dirs_l));
    let where_mode = raw.iter().any(|a| flag_matches(a, &where_s, &where_l));
    let case_sensitive = raw.iter().any(|a| flag_matches(a, &cs_s, &cs_l));

    #[cfg(windows)]
    let all_drives = raw.iter().any(|a| flag_matches(a, &drives_s, &drives_l));
    #[cfg(not(windows))]
    let all_drives = {
        let _ = (&drives_s, &drives_l);
        false
    };

    // Value flags
    let extension: Option<String> = raw
        .iter()
        .position(|a| flag_matches(a, &ext_s, &ext_l))
        .and_then(|i| raw.get(i + 1))
        .map(|s| s.trim_start_matches('.').to_lowercase());

    let dir: PathBuf = raw
        .iter()
        .position(|a| flag_matches(a, &dir_s, &dir_l))
        .and_then(|i| raw.get(i + 1))
        .map(PathBuf::from)
        .unwrap_or_else(|| ".".into());

    let threads: usize = raw
        .iter()
        .position(|a| flag_matches(a, &threads_s, &threads_l))
        .and_then(|i| raw.get(i + 1))
        .and_then(|v| v.parse().ok())
        .map(|n: usize| {
            if n > max_threads {
                eprintln!(
                    "Warning: {} threads requested but only {} logical cores available. Capping at {}.",
                    n, max_threads, max_threads
                );
                max_threads
            } else {
                n
            }
        })
        .unwrap_or(max_threads);

    let limit: Option<usize> = raw
        .iter()
        .position(|a| flag_matches(a, &limit_s, &limit_l))
        .and_then(|i| raw.get(i + 1))
        .and_then(|v| v.parse().ok());

    let exclude: Vec<String> = raw
        .iter()
        .position(|a| a == "--exclude")
        .and_then(|i| raw.get(i + 1))
        .map(|s| s.split(',').map(|p| p.trim().to_string()).collect())
        .unwrap_or_default();

    // Free arg (pattern)
    let value_flags = [
        ext_s.as_str(),
        ext_l.as_str(),
        dir_s.as_str(),
        dir_l.as_str(),
        threads_s.as_str(),
        threads_l.as_str(),
        limit_s.as_str(),
        limit_l.as_str(),
        "--exclude",
    ];

    let mut pattern: Option<String> = None;
    let mut skip_next = false;
    for arg in &raw {
        if skip_next {
            skip_next = false;
            continue;
        }
        if value_flags.contains(&arg.as_str()) {
            skip_next = true;
            continue;
        }
        if !arg.starts_with('-') {
            pattern = Some(arg.clone());
            break;
        }
    }

    // Validate unknown flags
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
            ]
            .iter()
            .map(|s| s.to_string()),
        )
        .collect();

    let mut skip_next = false;
    for arg in &raw {
        if skip_next {
            skip_next = false;
            continue;
        }
        if value_flags.contains(&arg.as_str()) {
            skip_next = true;
            continue;
        }
        if arg.starts_with('-') && !known_flags.contains(arg) {
            bail!("Unknown flag: {:?}. Run with --help for usage.", arg);
        }
    }

    // Validate flag combinations
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

    let limit = if first || where_mode { Some(1) } else { limit };

    Ok(ParsedFlags {
        pattern,
        dir,
        extension,
        threads,
        quiet,
        stats,
        all,
        verbose,
        open,
        dirs_only,
        where_mode,
        all_drives,
        case_sensitive,
        limit,
        exclude,
        show_help,
        show_version,
        show_config,
        edit_config,
        check_config,
        sync_config,
        reset_config,
    })
}
