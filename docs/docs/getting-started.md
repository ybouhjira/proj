---
sidebar_position: 1
---

# Getting Started

Get up and running with `proj` in minutes.

## Installation

Choose your preferred installation method:

### Cargo

```bash
cargo install proj-cli
```

### Homebrew

```bash
brew install ybouhjira/tap/proj
```

### Binary Download

Download the latest release for your platform:

```bash
# Linux
curl -L https://github.com/ybouhjira/proj/releases/latest/download/proj-linux-x64 -o proj
chmod +x proj
sudo mv proj /usr/local/bin/

# macOS
curl -L https://github.com/ybouhjira/proj/releases/latest/download/proj-macos-x64 -o proj
chmod +x proj
sudo mv proj /usr/local/bin/

# Windows
# Download from GitHub releases page
```

## Shell Integration

To enable the `proj cd` command, add shell integration:

### Bash

Add to your `~/.bashrc`:

```bash
eval "$(proj init bash)"
```

### Zsh

Add to your `~/.zshrc`:

```bash
eval "$(proj init zsh)"
```

### Fish

Add to your `~/.config/fish/config.fish`:

```fish
proj init fish | source
```

After adding, restart your shell or source the configuration file.

## First Steps

### Add Your First Project

```bash
# Add current directory
proj add .

# Add a project with a custom name
proj add ~/code/my-app --name my-app

# Add from a git URL (clones to configured directory)
proj add https://github.com/user/repo
```

### List Projects

```bash
proj list
```

### Navigate to a Project

```bash
# Using shell integration
proj cd my-app

# Without shell integration (prints path)
cd $(proj path my-app)
```

### Remove a Project

```bash
proj remove my-app
```

## Configuration

The configuration file is located at:
- Linux/macOS: `~/.config/proj/config.toml`
- Windows: `%APPDATA%\proj\config.toml`

Example configuration:

```toml
[projects]
root = "~/code"  # Default directory for cloning projects

[shell]
default = "zsh"

[ai]
# Coming soon: AI code quality checks
enabled = false
model = "claude-3-5-sonnet"
```

## Next Steps

- Learn about all available [commands](./commands.md)
- Customize your [configuration](./configuration.md)
- Set up [shell integration](./shell-integration.md)
