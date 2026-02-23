use anyhow::{Result, bail};
use colored::Colorize;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Open a file with the OS default handler
// ---------------------------------------------------------------------------

pub fn open_file(path: &std::path::Path) -> Result<()> {
    println!("{} {}", "Launching:".green().bold(), path.display());

    #[cfg(windows)]
    std::process::Command::new("cmd")
        .args(["/c", "start", "", &path.to_string_lossy()])
        .spawn()?;

    #[cfg(target_os = "macos")]
    std::process::Command::new("open").arg(path).spawn()?;

    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open").arg(path).spawn()?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Interactive picker when multiple matches found
// ---------------------------------------------------------------------------

pub fn prompt_and_open(paths: &[PathBuf]) -> Result<()> {
    println!(
        "{}",
        "Found more than 1 result! Pick one of the following:".yellow()
    );
    for (i, path) in paths.iter().enumerate() {
        println!("  [{}] {}", i + 1, path.display());
    }
    print!("\nEnter number to open (or q to quit): ");
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input == "q" || input == "Q" {
        return Ok(());
    }

    match input.parse::<usize>() {
        Ok(n) if n >= 1 && n <= paths.len() => {
            open_file(&paths[n - 1])?;
        }
        _ => {
            bail!("Invalid selection. Run ldx again to try.");
        }
    }

    Ok(())
}
