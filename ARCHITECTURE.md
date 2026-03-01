# proj — Architecture

## Overview

`proj` is a fast Rust CLI for managing all your projects across local directories and GitHub. It provides instant project switching, sync status tracking, fuzzy search, and AI-powered code quality checks.

## Commands

| Command | Description | Example |
|---------|-------------|---------|
| `proj ls` | List all projects (local + remote) | `proj ls`, `proj ls --remote` |
| `proj cd <name>` | Print path to project (shell wrapper does cd) | `proj cd face` → fuzzy matches `faceswap-api` |
| `proj clone <name>` | Clone a GitHub repo to projects dir | `proj clone solidkit` |
| `proj new <name>` | Create new project (dir + git + GitHub repo) | `proj new my-app --lang rust` |
| `proj sync` | Show sync status dashboard | `proj sync` |
| `proj open <name>` | Open project in editor/browser | `proj open solidkit --github` |
| `proj check <name>` | Run AI code quality checks | `proj check solidkit --all` |
| `proj info <name>` | Show detailed project info | `proj info solidkit` |

## Project Model

```rust
struct Project {
    name: String,
    local_path: Option<PathBuf>,       // ~/Projects/<name>
    github_repo: Option<GitHubRepo>,    // GitHub metadata
    sync_status: SyncStatus,            // synced, ahead, behind, diverged, local-only, remote-only
    git_status: Option<GitStatus>,      // dirty files, branch, last commit
}

struct GitHubRepo {
    full_name: String,          // owner/repo
    description: Option<String>,
    is_private: bool,
    default_branch: String,
    pushed_at: DateTime<Utc>,
    language: Option<String>,
    stars: u32,
    url: String,
}

enum SyncStatus {
    Synced,              // local matches remote
    LocalAhead(u32),     // local has N unpushed commits
    RemoteBehind(u32),   // remote has N unmerged commits
    Diverged,            // both have unique commits
    LocalOnly,           // no remote
    RemoteOnly,          // not cloned
    NoGit,               // local dir without git
}

struct GitStatus {
    branch: String,
    dirty_files: u32,
    last_commit_msg: String,
    last_commit_date: DateTime<Utc>,
}
```

## Directory Structure

```
src/
├── main.rs              # Entry point, clap CLI definition
├── cli.rs               # CLI argument structs (clap derive)
├── commands/
│   ├── mod.rs
│   ├── list.rs          # proj ls
│   ├── cd.rs            # proj cd (outputs path for shell wrapper)
│   ├── clone.rs         # proj clone
│   ├── new.rs           # proj new
│   ├── sync.rs          # proj sync (dashboard)
│   ├── open.rs          # proj open
│   ├── check.rs         # proj check (AI quality)
│   └── info.rs          # proj info
├── config.rs            # ~/.config/proj/config.toml
├── github.rs            # GitHub API via `gh` CLI
├── discovery.rs         # Scan local dirs + GitHub repos
├── project.rs           # Project model + SyncStatus
├── fuzzy.rs             # Fuzzy matching
└── ui.rs                # Terminal output (tables, colors, status)
```

## Configuration

`~/.config/proj/config.toml`:
```toml
# Directory containing local projects
projects_dir = "~/Projects"

# GitHub username for repo listing
github_user = "ybouhjira"

# Default editor for `proj open`
editor = "code"

# Show private repos in listing
show_private = true

# AI checks configuration
[checks]
provider = "claude"     # claude | openai | local
checks = ["quality", "logging", "testing", "security"]
```

## Shell Integration

`proj cd` can't change the parent shell's directory, so we output the path and use a shell function:

```bash
# Added to ~/.bashrc or ~/.zshrc by `proj init`
proj() {
    if [ "$1" = "cd" ]; then
        shift
        local dir=$(command proj cd "$@")
        if [ $? -eq 0 ] && [ -n "$dir" ]; then
            builtin cd "$dir"
        fi
    else
        command proj "$@"
    fi
}
```

`proj init bash|zsh|fish` outputs the shell wrapper for the user to add.

## GitHub Integration

We shell out to `gh` CLI (already authenticated) rather than using octocrab. This is simpler, more reliable, and avoids token management:

```rust
// Uses: gh repo list --json name,description,...
// Uses: gh repo create
// Uses: gh repo view
// Uses: gh issue list/create
```

## AI Code Quality Checks (proj check)

Inspired by yb-coverage but broader. Defines check interfaces:

```rust
trait QualityCheck {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn run(&self, project_path: &Path) -> CheckResult;
}

struct CheckResult {
    score: f32,           // 0.0 - 1.0
    issues: Vec<Issue>,
    suggestions: Vec<String>,
}
```

Built-in checks:
- **quality** — code quality, complexity, patterns
- **logging** — logging consistency, levels, structured logging
- **testing** — test coverage, test quality, missing tests
- **security** — OWASP checks, dependency audit, secrets scan
- **docs** — documentation completeness, API docs

Checks can use Claude CLI (`claude -p "analyze..."`) or local tools (clippy, cargo-audit).

## Key Crates

| Crate | Purpose |
|-------|---------|
| clap (derive) | CLI parsing |
| tokio | Async runtime |
| serde + toml | Config parsing |
| tabled | Table output |
| console + indicatif | Colors, progress bars |
| nucleo-matcher | Fuzzy matching |
| chrono | Date handling |
| dirs | XDG directories |
| anyhow | Error handling |
| which | Check for `gh` CLI |

## Output Style

```
$ proj ls

  📦 Projects (18 local · 200 remote)

  NAME               STATUS    BRANCH   DIRTY  LAST PUSH
  faceswap-api       ✅ synced  main     10∆    2h ago
  solidkit           ⬆ ahead   main     1∆     3d ago
  d3-wysiwyg         ⬆ ahead   master   8∆     3d ago
  voiceswap-desktop  ✅ synced  main     0∆     12h ago
  claude-headless    📡 custom  main     5∆     1h ago
  puzzle             📡 ext     master   0∆     —
  app-hub            💻 no-git  —        —      —
  ...

  Remote only (182):  proj ls --remote to see all
```

```
$ proj sync

  🔄 Sync Dashboard

  ⬆ Need push (2):
    solidkit         main  +1 commit
    d3-wysiwyg       master  +3 commits

  ⚠ Dirty (4):
    faceswap-api     10 files
    d3-wysiwyg       8 files
    claude-headless  5 files
    solidkit         1 file

  💻 No git (6):
    app-hub, dfl-face-filter, learning-odoo, linkedin, ...

  ✅ Clean (2):
    voiceswap-desktop, puzzle
```
