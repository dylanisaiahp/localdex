use crate::config::{FlagDef, LdxConfig, is_flag_available};
use colored::Colorize;

// ---------------------------------------------------------------------------
// Number formatting
// ---------------------------------------------------------------------------

pub fn fmt_num(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

// ---------------------------------------------------------------------------
// Help output
// ---------------------------------------------------------------------------

pub fn print_help(config: &LdxConfig) {
    // ── Header ───────────────────────────────────────────────────────────────
    println!();
    println!(
        "  {}  {}",
        "ldx".bright_cyan().bold(),
        "blazing-fast parallel file search".dimmed()
    );
    println!(
        "  {}",
        format!(
            "v{} · github.com/dylanisaiahp/localdex",
            env!("CARGO_PKG_VERSION")
        )
        .dimmed()
    );
    println!();

    // ── Usage ─────────────────────────────────────────────────────────────────
    println!(
        "  {} {} {}",
        "Usage:".bold(),
        "ldx".bright_cyan(),
        "[pattern] [options]".dimmed()
    );
    println!();

    // ── Examples ──────────────────────────────────────────────────────────────
    println!("  {}", "Examples:".bold());
    let examples: &[(&str, &str)] = &[
        ("ldx invoice", "substring match in filename"),
        ("ldx -e rs -d ~/projects -q", "count .rs files in a dir"),
        ("ldx -o -1 vintagestory", "find and launch a file"),
        ("ldx -D -w localdex", "find a dir, print cd hint"),
        (
            "ldx -e log -L 5 --exclude target,node_modules",
            "limit results, skip dirs",
        ),
    ];

    for (cmd, desc) in examples {
        println!("    {:<50} {}", cmd.bright_cyan(), desc.dimmed());
    }
    println!();

    // ── Flags ─────────────────────────────────────────────────────────────────
    println!("  {}", "Flags:".bold());

    let mut flags: Vec<&FlagDef> = config
        .flags
        .values()
        .filter(|f| is_flag_available(f))
        .collect();
    flags.sort_by(|a, b| a.long.cmp(&b.long));

    for flag in &flags {
        println!(
            "    {}  {:<28} {}",
            format!("-{}", flag.short).bright_cyan(),
            format!("--{}", flag.long).cyan(),
            flag.description.dimmed()
        );
    }

    // Management flags (not in config)
    let mgmt: &[(&str, &str)] = &[
        ("--check", "Validate config"),
        ("--config", "Show config path"),
        ("--edit", "Open config in editor"),
        ("--reset", "Restore default flags"),
        ("--sync", "Add missing default flags"),
        ("--version", "Show version"),
    ];
    println!();
    println!("  {}", "Management:".bold());
    for (flag, desc) in mgmt {
        println!(
            "    {}  {:<28} {}",
            "  ".dimmed(),
            flag.cyan(),
            desc.dimmed()
        );
    }

    // ── User aliases ──────────────────────────────────────────────────────────
    if !config.aliases.is_empty() {
        println!();
        println!("  {}", "Your Aliases:".bold());

        let mut aliases: Vec<(&String, &String)> = config.aliases.iter().collect();
        aliases.sort_by_key(|(k, _)| k.as_str());

        for (name, expansion) in &aliases {
            println!(
                "    {:<16} {}  {}",
                name.bright_cyan(),
                "→".dimmed(),
                expansion.dimmed()
            );
        }
    }

    // ── User custom flags ─────────────────────────────────────────────────────
    if !config.custom.is_empty() {
        println!();
        println!("  {}", "Your Custom Flags:".bold());

        let mut custom: Vec<(&String, &crate::config::FlagDef)> = config.custom.iter().collect();
        custom.sort_by_key(|(_, f)| f.long.as_str());

        for (_, flag) in &custom {
            println!(
                "    {}  {:<28} {}",
                format!("-{}", flag.short).bright_cyan(),
                format!("--{}", flag.long).cyan(),
                flag.description.dimmed()
            );
        }
    }

    // ── Tips ──────────────────────────────────────────────────────────────────
    println!();
    println!(
        "  {}  {}",
        "Tip:".bold(),
        "-d = where to search  -D = find dirs  -s = case-sensitive  -S = stats  run --edit to customize".dimmed()
    );
    println!();
}

// ---------------------------------------------------------------------------
// Print scan result summary line
// ---------------------------------------------------------------------------

pub fn print_result(
    result: &crate::search::ScanResult,
    reported_matches: usize,
    f: &crate::flags::ParsedFlags,
    indent: &str,
) {
    if f.all {
        println!(
            "{}Found {} file{} in {:.3}s",
            indent,
            fmt_num(reported_matches),
            if reported_matches == 1 { "" } else { "s" },
            result.duration.as_secs_f64()
        );
    } else if f.dirs_only {
        println!(
            "{}Found {} matching director{} in {:.3}s",
            indent,
            fmt_num(reported_matches),
            if reported_matches == 1 { "y" } else { "ies" },
            result.duration.as_secs_f64()
        );
    } else {
        println!(
            "{}Found {} matching file{} in {:.3}s",
            indent,
            fmt_num(reported_matches),
            if reported_matches == 1 { "" } else { "s" },
            result.duration.as_secs_f64()
        );
    }
}

// ---------------------------------------------------------------------------
// Print stats line
// ---------------------------------------------------------------------------

pub fn print_stats(
    result: &crate::search::ScanResult,
    f: &crate::flags::ParsedFlags,
    indent: &str,
) {
    let s = result.duration.as_secs_f64();
    if !f.stats || s <= 0.0 {
        return;
    }
    let tc = result.files + result.dirs;
    if f.verbose {
        println!(
            "{}Scanned {} entries ({} files + {} dirs) | Speed: {} entries/s | Threads: {}",
            indent,
            fmt_num(tc),
            fmt_num(result.files),
            fmt_num(result.dirs),
            fmt_num((tc as f64 / s) as usize),
            f.threads
        );
    } else {
        println!(
            "{}Scanned {} entries | {} entries/s | Threads: {}",
            indent,
            fmt_num(tc),
            fmt_num((tc as f64 / s) as usize),
            f.threads
        );
    }
}
