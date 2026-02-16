use crate::config::{FlagDef, LdxConfig, is_flag_available};

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
    println!(
        "Usage: ldx [pattern] [options]\n\
        \n\
        Examples:\n\
          ldx invoice.pdf                  # basename substring search\n\
          ldx -e pdf -q                    # count all .pdf files quietly\n\
          ldx rs -d D:\\Development        # find files with 'rs' in name\n\
          ldx -a -S -d C:\\               # count every file on C:\\\n\
          ldx -a -A -S                     # count every file on all drives\n\
        \n\
        Options:"
    );

    let mut flags: Vec<&FlagDef> = config
        .flags
        .values()
        .filter(|f| is_flag_available(f))
        .collect();

    flags.sort_by(|a, b| a.long.cmp(&b.long));

    for flag in flags {
        println!(
            "  -{}, --{:<20} {}",
            flag.short, flag.long, flag.description
        );
    }

    println!("  --version                    Show version");
}
