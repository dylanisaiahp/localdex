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
    println!("  {}", "v0.0.7 · github.com/dylanisaiahp/localdex".dimmed());
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
        ("ldx invoice", "substring match on filename"),
        ("ldx -e pdf -q", "count all .pdf files quietly"),
        ("ldx -e rs -d ~/projects", "find .rs files in a directory"),
        ("ldx -a -S -d C:\\", "count every file on C:\\ with stats"),
        ("ldx vintagestory -o -1", "find and launch a file"),
        ("ldx localdex -D -w", "find a directory, print cd hint"),
        ("ldx -e log -L 5", "stop after 5 matches"),
        (
            "ldx main.rs --exclude target",
            "exclude a directory from scan",
        ),
    ];

    for (cmd, desc) in examples {
        println!("    {:<42} {}", cmd.bright_cyan(), desc.dimmed());
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
        ("--check", "Validate config and print summary"),
        ("--config", "Print config file location"),
        ("--edit", "Open config in default editor"),
        (
            "--reset",
            "Restore default flags (preserves aliases & custom)",
        ),
        ("--sync", "Add missing default flags to config"),
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
    println!("  {}", "Tips:".bold());
    println!(
        "    {}",
        "-d sets where to search.  -D searches for directories.".dimmed()
    );
    println!(
        "    {}",
        "-s is case-sensitive.     -S shows stats.".dimmed()
    );
    println!(
        "    {}",
        "-e pdf matches .pdf files. A bare pattern matches filenames.".dimmed()
    );
    println!(
        "    {}",
        "Edit config to remap flags, add aliases, or define custom flags.".dimmed()
    );
    println!();
}
