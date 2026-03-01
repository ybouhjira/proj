# proj

**Fast CLI for managing all your projects — local + GitHub sync, fuzzy search, instant navigation**

[![Crates.io](https://img.shields.io/crates/v/proj?style=flat-square)](https://crates.io/crates/proj)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)
[![CI](https://img.shields.io/github/actions/workflow/status/ybouhjira/proj/ci.yml?style=flat-square)](https://github.com/ybouhjira/proj/actions)

## Why proj?

- **You have dozens of projects scattered across your machine** — some synced, some dirty, some forgotten
- **Switching between projects is slow** — `cd ~/Projects/that-one-repo-i-worked-on-last-month` gets old
- **You lose track of what needs pushing** — dirty repos pile up, you forget what's on your machine vs GitHub

`proj` fixes this. One tool to see everything, sync everything, and jump anywhere instantly.

## Demo

<!-- TODO: Add VHS recording -->

```bash
$ proj ls
  📦 Projects (18 local · 327 remote)

 NAME                    STATUS      BRANCH   DIRTY   LAST PUSH
 faceswap-api            ✅ synced   main     10∆     15h ago
 solidkit                ✅ synced   main     5∆      16h ago
 d3-wysiwyg              ✅ synced   master   8∆      1d ago
 voiceswap-desktop       ✅ synced   main     0∆      2h ago
 app-hub                 📁 no-git   —        —       2w ago

$ proj cd face
# Instantly jumps to /home/user/Projects/faceswap-api

$ proj sync
  🔄 Sync Dashboard

  ⚠ Dirty (4):
    faceswap-api main  10 files
    d3-wysiwyg master  8 files

  💻 No git (11):
    app-hub, dfl-face-filter, ...

  ✅ Clean (3):
    voiceswap-desktop main
```

## Features

- 🚀 **Instant fuzzy search** — `proj cd face` finds `faceswap-api` in milliseconds
- 📊 **Unified dashboard** — See all local + GitHub repos in one view
- 🔄 **Smart sync** — Know which repos are dirty, ahead, behind, or untracked
- 🎯 **Quality checks** — Run linters, tests, and custom checks across projects
- 🌐 **GitHub integration** — Clone, create, and browse repos without leaving terminal
- ⚡ **Blazing fast** — Written in Rust, sub-second response times
- 🔍 **Rich metadata** — Language, stars, last push time, dirty file count
- 🛠️ **Shell integration** — `cd` wrapper for instant navigation

## Quick Start

### Installation

```bash
# From crates.io
cargo install proj

# From source
git clone https://github.com/ybouhjira/proj
cd proj
cargo install --path .
```

### Setup

1. Initialize shell integration:

```bash
# Add to ~/.bashrc or ~/.zshrc
eval "$(proj init bash)"  # or 'zsh'
```

2. Configure your projects directory (optional):

```bash
# Create ~/.config/proj/config.toml
mkdir -p ~/.config/proj
cat > ~/.config/proj/config.toml << EOF
projects_dir = "~/Projects"
github_username = "yourusername"
EOF
```

3. Start using:

```bash
proj ls              # List all projects
proj cd myproject    # Jump to a project (fuzzy search)
proj sync            # See what needs attention
```

## Commands

| Command | Description |
|---------|-------------|
| `proj ls` | List all projects (local + remote) with status |
| `proj cd <query>` | Fuzzy search and jump to project directory |
| `proj sync` | Show sync status dashboard (dirty, ahead, behind) |
| `proj clone <name>` | Clone a GitHub repo to projects directory |
| `proj new <name>` | Create new project locally + GitHub repo |
| `proj open <name>` | Open project in browser/editor |
| `proj info <name>` | Show detailed project information |
| `proj check [name]` | Run quality checks (linters, tests) |
| `proj init <shell>` | Generate shell integration script |

### Options

- `proj ls --local` — Show only local projects
- `proj ls --remote` — Show only GitHub repos
- `proj ls --all` — Show all (default)
- `proj new --public` — Create public GitHub repo (default: private)
- `proj open --github` — Open GitHub page in browser
- `proj open --dir` — Open project directory in file manager
- `proj check --all` — Run checks on all projects

## Comparison

| Feature | proj | ghq | gita | mani |
|---------|------|-----|------|------|
| Unified local + remote view | ✅ | ❌ | ❌ | ❌ |
| Fuzzy search navigation | ✅ | ⚠️ (via fzf) | ❌ | ❌ |
| Dirty file tracking | ✅ | ❌ | ✅ | ❌ |
| GitHub integration | ✅ | ⚠️ (basic) | ❌ | ❌ |
| Quality checks | ✅ | ❌ | ❌ | ❌ |
| Sync dashboard | ✅ | ❌ | ⚠️ (basic) | ❌ |
| Written in Rust | ✅ | ✅ | ❌ (Python) | ❌ (Go) |
| Shell CD integration | ✅ | ✅ | ❌ | ❌ |

`proj` combines the best of all worlds: fast like `ghq`, smart like `gita`, and GitHub-native.

## Configuration

Create `~/.config/proj/config.toml`:

```toml
# Where your projects live
projects_dir = "~/Projects"

# Your GitHub username (for listing remote repos)
github_username = "ybouhjira"

# Patterns to exclude from discovery
exclude_patterns = [
    "node_modules",
    "target",
    ".git",
    "vendor"
]

# Custom quality checks
[checks]
rust = ["cargo clippy", "cargo test"]
typescript = ["npm run lint", "npm test"]
python = ["ruff check", "pytest"]
```

## Shell Integration

The `cd` wrapper enables instant navigation:

```bash
# Instead of:
cd ~/Projects/faceswap-api

# Just type:
proj cd face
```

### How it works

`proj init` generates a shell function that:
1. Runs `proj cd <query>` to find the project
2. Reads the output path
3. Changes directory using the shell's built-in `cd`

### Manual setup

If `eval "$(proj init bash)"` doesn't work, add this to `~/.bashrc`:

```bash
proj() {
    if [ "$1" = "cd" ]; then
        local result=$(command proj cd "$2")
        if [ -d "$result" ]; then
            cd "$result"
        else
            echo "$result"
        fi
    else
        command proj "$@"
    fi
}
```

For Zsh, replace `~/.bashrc` with `~/.zshrc`.

## Roadmap

**v0.2** (planned):
- 🤖 AI-powered code quality suggestions
- 🏷️ Project tags and filtering
- 📈 Activity statistics and insights
- 👀 Watch mode for continuous sync monitoring
- 🔗 GitLab and Bitbucket support
- 📦 Multi-directory project roots
- 🎨 Custom themes and output formats

Want a feature? [Open an issue](https://github.com/ybouhjira/proj/issues)!

## Contributing

Contributions welcome! Here's how:

1. **Fork the repo** — `gh repo fork ybouhjira/proj --clone`
2. **Create a branch** — `git checkout -b feature/my-feature`
3. **Make changes** — Follow Rust style guidelines (`cargo fmt`, `cargo clippy`)
4. **Add tests** — All new features need tests
5. **Submit PR** — Include a clear description of the change

### Development

```bash
# Run tests
cargo test

# Run with debug output
cargo run -- ls --local

# Build release binary
cargo build --release

# Install locally
cargo install --path .
```

## License

MIT License - see [LICENSE](LICENSE) file for details.

---

Built with ⚡ by [Youssef Bouhjira](https://github.com/ybouhjira)

Star the repo if `proj` saves you time!
