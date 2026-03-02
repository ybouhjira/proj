#![allow(dead_code)]

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

mod cache;
mod checks;
mod cli;
mod commands;
mod config;
mod discovery;
mod fuzzy;
mod github;
mod project;
mod ui;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_env("PROJ_LOG")
                .unwrap_or_else(|_| EnvFilter::new("off"))
        )
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();

    github::check_gh_cli().await?;

    let cli = Cli::parse();

    match cli.command {
        Commands::List {
            remote,
            local,
            all,
            sort,
            refresh,
        } => {
            commands::list::execute(remote, local, all, &sort, refresh).await?;
        }
        Commands::Cd { query } => {
            commands::cd::execute(query.as_deref()).await?;
        }
        Commands::Clone { name } => {
            commands::clone::execute(&name).await?;
        }
        Commands::New { name, public, lang } => {
            commands::new_project::execute(&name, public, lang).await?;
        }
        Commands::Sync => {
            commands::sync::execute().await?;
        }
        Commands::Open { name, github, dir } => {
            commands::open::execute(name.as_deref(), github, dir).await?;
        }
        Commands::Check {
            name,
            all,
            check,
            ai,
        } => {
            commands::check::execute(&name, all, check, ai).await?;
        }
        Commands::Info { name } => {
            commands::info::execute(&name).await?;
        }
        Commands::Init { shell } => {
            commands::init::execute(&shell).await?;
        }
        Commands::Completions { shell } => {
            commands::completions::execute(&shell).await?;
        }
    }

    Ok(())
}
