use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::io;

pub async fn execute(shell_name: &str) -> Result<()> {
    let shell = match shell_name {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "powershell" => Shell::PowerShell,
        _ => {
            anyhow::bail!(
                "Unsupported shell: {}. Use: bash, zsh, fish, powershell",
                shell_name
            )
        }
    };

    let mut cmd = crate::cli::Cli::command();
    generate(shell, &mut cmd, "proj", &mut io::stdout());

    Ok(())
}
