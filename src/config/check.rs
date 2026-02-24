use anyhow::Result;
use colored::Colorize;
use std::collections::HashMap;

use crate::config::{DEFAULT_CONFIG, LdxConfig, config_path, is_flag_available};

// ---------------------------------------------------------------------------
// --check: validate config and print a summary
// ---------------------------------------------------------------------------

pub fn check_config(config: &LdxConfig) {
    let path = config_path();
    println!(
        "{}",
        "── Config Check ─────────────────────────────────"
            .bright_cyan()
            .bold()
    );
    println!("  Path    : {}", path.display());

    let defaults: LdxConfig = toml::from_str(DEFAULT_CONFIG).expect("DEFAULT_CONFIG is valid TOML");
    let default_targets: Vec<&str> = defaults
        .flags
        .values()
        .filter_map(|f| f.target.as_deref())
        .collect();

    let mut warnings = 0usize;

    // ── Flags ──
    println!();
    println!("  {} {} defined", "Flags:".bold(), config.flags.len());
    let mut seen_shorts: HashMap<&str, &str> = HashMap::new();
    let mut seen_longs: HashMap<&str, &str> = HashMap::new();

    for (key, flag) in &config.flags {
        let available = is_flag_available(flag);

        if let Some(prior) = seen_shorts.get(flag.short.as_str()) {
            println!(
                "    {} short '-{}' on [flags.{}] already used by [flags.{}]",
                "WARN".yellow().bold(),
                flag.short,
                key,
                prior
            );
            warnings += 1;
        } else {
            seen_shorts.insert(&flag.short, key.as_str());
        }

        if let Some(prior) = seen_longs.get(flag.long.as_str()) {
            println!(
                "    {} long '--{}' on [flags.{}] already used by [flags.{}]",
                "WARN".yellow().bold(),
                flag.long,
                key,
                prior
            );
            warnings += 1;
        } else {
            seen_longs.insert(&flag.long, key.as_str());
        }

        if flag.target.is_none() && flag.action.as_deref() != Some("show_help") {
            println!(
                "    {} [flags.{}] has no 'target' — flag will be ignored",
                "WARN".yellow().bold(),
                key
            );
            warnings += 1;
        }

        if let Some(target) = &flag.target
            && target != "help"
            && !default_targets.contains(&target.as_str())
        {
            println!(
                "    {} [flags.{}] target '{}' is not a known internal target",
                "WARN".yellow().bold(),
                key,
                target
            );
            warnings += 1;
        }

        if !available {
            println!(
                "    {} [flags.{}] — os='{}' (not active on this platform)",
                "skip".dimmed(),
                key,
                flag.os
            );
        }
    }

    // ── Aliases ──
    println!();
    if config.aliases.is_empty() {
        println!("  {} none defined", "Aliases:".bold());
    } else {
        println!("  {} {} defined", "Aliases:".bold(), config.aliases.len());
        for (name, expansion) in &config.aliases {
            println!("    {} → {}", name.cyan(), expansion.dimmed());
        }
    }

    // ── Custom flags ──
    println!();
    if config.custom.is_empty() {
        println!("  {} none defined", "Custom flags:".bold());
    } else {
        println!(
            "  {} {} defined",
            "Custom flags:".bold(),
            config.custom.len()
        );
        for (key, flag) in &config.custom {
            if flag.action.is_none() {
                println!(
                    "    {} [custom.{}] has no 'action' field",
                    "WARN".yellow().bold(),
                    key
                );
                warnings += 1;
            }
            if flag.target.is_none() {
                println!(
                    "    {} [custom.{}] has no 'target' field",
                    "WARN".yellow().bold(),
                    key
                );
                warnings += 1;
            }
            let target = flag.target.as_deref().unwrap_or("?");
            let value = flag.value.as_deref().unwrap_or("");
            println!(
                "    -{} / --{:<18} {} → {} {}",
                flag.short,
                flag.long,
                flag.description.dimmed(),
                target.cyan(),
                if value.is_empty() {
                    String::new()
                } else {
                    format!("= {}", value.bright_white())
                }
            );
        }
    }

    // ── Missing vs defaults ──
    let missing: Vec<&str> = default_targets
        .iter()
        .copied()
        .filter(|t| {
            *t != "help"
                && !config
                    .flags
                    .values()
                    .any(|f| f.target.as_deref() == Some(t))
        })
        .collect();

    if !missing.is_empty() {
        println!();
        println!(
            "  {} flags missing vs defaults (run --sync to restore):",
            "Missing:".yellow().bold()
        );
        for m in &missing {
            println!("    • {}", m);
        }
        warnings += missing.len();
    }

    // ── Summary ──
    println!();
    if warnings == 0 {
        println!("  {} Config looks great!", "OK".green().bold());
    } else {
        println!(
            "  {} {} warning{} found",
            "!!".yellow().bold(),
            warnings,
            if warnings == 1 { "" } else { "s" }
        );
    }
    println!(
        "{}",
        "─────────────────────────────────────────────────"
            .bright_cyan()
            .bold()
    );
}

// ---------------------------------------------------------------------------
// --sync
// ---------------------------------------------------------------------------

pub fn sync_config() -> Result<()> {
    let path = config_path();
    let contents = std::fs::read_to_string(&path)?;
    let user: LdxConfig = toml::from_str(&contents)?;
    let defaults: LdxConfig = toml::from_str(DEFAULT_CONFIG)?;

    let user_targets: Vec<String> = user
        .flags
        .values()
        .filter_map(|f| f.target.clone())
        .collect();
    let user_keys: Vec<&String> = user.flags.keys().collect();

    let mut added = 0usize;
    let mut appended = String::new();

    for (key, flag) in &defaults.flags {
        if user_keys.contains(&key) {
            continue;
        }
        if let Some(target) = &flag.target
            && user_targets.iter().any(|t| t == target)
        {
            continue;
        }
        let target = match &flag.target {
            Some(t) => t,
            None => continue,
        };
        let action = flag.action.as_deref().unwrap_or("set_boolean");
        appended.push_str(&format!(
            "\n[flags.{}]\nshort = \"{}\"\nlong = \"{}\"\ndescription = \"{}\"\nos = \"{}\"\naction = \"{}\"\ntarget = \"{}\"\n",
            key, flag.short, flag.long, flag.description, flag.os, action, target
        ));
        println!(
            "  {} added [flags.{}]  (-{} / --{})",
            "+".green().bold(),
            key,
            flag.short,
            flag.long
        );
        added += 1;
    }

    if added == 0 {
        println!(
            "{}",
            "Config is already up to date — nothing to sync.".green()
        );
        return Ok(());
    }

    let new_contents = format!("{}\n{}", contents.trim_end(), appended);
    std::fs::write(&path, new_contents)?;
    println!(
        "{} {} flag{} added.",
        "Synced:".green().bold(),
        added,
        if added == 1 { "" } else { "s" }
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// --reset
// ---------------------------------------------------------------------------

pub fn reset_config() -> Result<()> {
    let path = config_path();

    print!(
        "{} This will restore all [flags] to defaults.\n  Your [aliases] and [custom] flags will be preserved.\n  Continue? [y/N] ",
        "Reset:".yellow().bold()
    );
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
        println!("Reset cancelled.");
        return Ok(());
    }

    let user_contents = std::fs::read_to_string(&path)?;
    let user: LdxConfig = toml::from_str(&user_contents)?;
    let defaults: LdxConfig = toml::from_str(DEFAULT_CONFIG)?;

    let meta_block: String = {
        let mut in_meta = false;
        let mut lines = Vec::new();
        for line in user_contents.lines() {
            if line.starts_with("[meta]") {
                in_meta = true;
            }
            if in_meta {
                if line.starts_with('[') && !line.starts_with("[meta]") {
                    break;
                }
                lines.push(line);
            }
        }
        lines.join("\n")
    };

    let mut out = String::from(
        "# localdex configuration\n\
         # Flags reset to defaults — your aliases and custom flags have been preserved.\n\n",
    );

    for (key, flag) in &defaults.flags {
        let action = flag.action.as_deref().unwrap_or("set_boolean");
        let target_line = flag
            .target
            .as_deref()
            .map(|t| format!("\ntarget = \"{}\"", t))
            .unwrap_or_default();
        out.push_str(&format!(
            "[flags.{}]\nshort = \"{}\"\nlong = \"{}\"\ndescription = \"{}\"\nos = \"{}\"\naction = \"{}\"{}\n\n",
            key, flag.short, flag.long, flag.description, flag.os, action, target_line
        ));
    }

    out.push_str("[aliases]\n");
    for (name, expansion) in &user.aliases {
        out.push_str(&format!("{} = {:?}\n", name, expansion));
    }
    out.push('\n');

    for (key, flag) in &user.custom {
        out.push_str(&format!(
            "[custom.{}]\nshort = \"{}\"\nlong = \"{}\"\ndescription = \"{}\"\nos = \"{}\"\n",
            key, flag.short, flag.long, flag.description, flag.os
        ));
        if let Some(action) = &flag.action {
            out.push_str(&format!("action = \"{}\"\n", action));
        }
        if let Some(target) = &flag.target {
            out.push_str(&format!("target = \"{}\"\n", target));
        }
        if let Some(value) = &flag.value {
            out.push_str(&format!("value = \"{}\"\n", value));
        }
        out.push('\n');
    }

    if !meta_block.is_empty() {
        out.push_str(&meta_block);
        out.push('\n');
    }

    std::fs::write(&path, out)?;
    println!(
        "{} Flags reset to defaults. {} alias{} and {} custom flag{} preserved.",
        "Done:".green().bold(),
        user.aliases.len(),
        if user.aliases.len() == 1 { "" } else { "es" },
        user.custom.len(),
        if user.custom.len() == 1 { "" } else { "s" }
    );
    Ok(())
}
