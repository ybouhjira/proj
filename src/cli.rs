use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "proj")]
#[command(about = "Fast CLI for managing all your projects — local + GitHub sync, fuzzy search, code quality checks", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "List all projects (local + remote)")]
    #[command(visible_alias = "ls")]
    List {
        #[arg(long, help = "Show remote-only projects")]
        remote: bool,

        #[arg(long, help = "Show only local projects")]
        local: bool,

        #[arg(long, help = "Show all projects (local + remote)")]
        all: bool,
    },

    #[command(about = "Print path to project for shell wrapper (use fuzzy search)")]
    Cd {
        #[arg(help = "Project name or fuzzy query (omit for interactive picker)")]
        query: Option<String>,
    },

    #[command(about = "Clone a GitHub repo to projects directory")]
    Clone {
        #[arg(help = "Repository name (fuzzy match)")]
        name: String,
    },

    #[command(about = "Create new project (directory + git + GitHub repo)")]
    New {
        #[arg(help = "Project name")]
        name: String,

        #[arg(long, help = "Create public repo (default: private)")]
        public: bool,

        #[arg(long, help = "Language for scaffolding (rust, js, py, go)")]
        lang: Option<String>,
    },

    #[command(about = "Show sync status dashboard")]
    Sync,

    #[command(about = "Open project in editor or browser")]
    Open {
        #[arg(help = "Project name (fuzzy match)")]
        name: String,

        #[arg(long, help = "Open GitHub URL in browser")]
        github: bool,

        #[arg(long, help = "Open directory in file manager")]
        dir: bool,
    },

    #[command(about = "Run AI-powered code quality checks")]
    Check {
        #[arg(help = "Project name (fuzzy match)")]
        name: String,

        #[arg(long, help = "Run all checks")]
        all: bool,

        #[arg(long, help = "Run specific check (quality, logging, testing, security, docs)")]
        check: Option<String>,
    },

    #[command(about = "Show detailed project info")]
    Info {
        #[arg(help = "Project name (fuzzy match)")]
        name: String,
    },

    #[command(about = "Output shell wrapper function for cd command")]
    Init {
        #[arg(help = "Shell type (bash, zsh, fish)")]
        shell: String,
    },
}
