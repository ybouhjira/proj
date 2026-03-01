---
sidebar_position: 2
---

# Commands Reference

Complete reference for all `proj` commands.

## Core Commands

### `proj add`

Add a project to the registry.

```bash
# Add current directory
proj add .

# Add specific directory
proj add /path/to/project

# Add with custom name
proj add . --name my-project

# Clone and add from git URL
proj add https://github.com/user/repo

# Add with tags
proj add . --tags rust,cli
```

**Options:**
- `--name <NAME>` - Custom project name (default: directory name)
- `--tags <TAGS>` - Comma-separated tags for organization

---

### `proj list`

List all registered projects.

```bash
# List all projects
proj list

# List with details (path, tags)
proj list --verbose

# Filter by tag
proj list --tag rust

# Output as JSON
proj list --json
```

**Options:**
- `-v, --verbose` - Show detailed information
- `--tag <TAG>` - Filter by tag
- `--json` - Output in JSON format

---

### `proj cd`

Change directory to a project (requires shell integration).

```bash
# Navigate to project
proj cd my-project

# Fuzzy search
proj cd proj  # Matches "my-project", "proj-cli", etc.
```

**Note:** This command requires shell integration. See [Shell Integration](./shell-integration.md).

---

### `proj path`

Print the path to a project.

```bash
# Get project path
proj path my-project

# Use with cd (without shell integration)
cd $(proj path my-project)

# Open in editor
code $(proj path my-project)
```

---

### `proj remove`

Remove a project from the registry.

```bash
# Remove by name
proj remove my-project

# Remove multiple projects
proj remove project1 project2 project3
```

**Note:** This only removes from the registry, not from disk.

---

### `proj search`

Search for projects by name or tag.

```bash
# Search by name (fuzzy)
proj search cli

# Search by tag
proj search --tag rust

# Combined search
proj search web --tag typescript
```

**Options:**
- `--tag <TAG>` - Filter by tag

---

## Utility Commands

### `proj init`

Generate shell integration script.

```bash
# Bash
proj init bash

# Zsh
proj init zsh

# Fish
proj init fish
```

See [Shell Integration](./shell-integration.md) for setup instructions.

---

### `proj config`

Manage configuration.

```bash
# Show current configuration
proj config show

# Edit configuration file
proj config edit

# Get specific value
proj config get projects.root

# Set specific value
proj config set projects.root ~/code
```

**Options:**
- `show` - Display current configuration
- `edit` - Open config file in default editor
- `get <KEY>` - Get configuration value
- `set <KEY> <VALUE>` - Set configuration value

---

### `proj check`

Run code quality checks (coming soon).

```bash
# Check current project
proj check

# Check specific files
proj check src/main.rs

# Check with specific model
proj check --model claude-3-5-sonnet
```

**Status:** This feature is in development. See [roadmap](https://github.com/ybouhjira/proj/issues).

---

## Global Options

All commands support these global options:

- `-h, --help` - Show help information
- `-V, --version` - Show version information
- `--config <PATH>` - Use custom config file
- `--no-color` - Disable colored output
- `-q, --quiet` - Suppress non-error output
- `-v, --verbose` - Enable verbose logging

## Examples

### Typical Workflow

```bash
# Clone and add a new project
proj add https://github.com/rust-lang/rust

# List all projects
proj list

# Navigate to it
proj cd rust

# Search for rust projects
proj search --tag rust

# Remove old projects
proj remove old-project
```

### Working with Tags

```bash
# Add projects with tags
proj add . --tags rust,cli,tool
proj add ~/web/app --tags typescript,react,web

# List projects by tag
proj list --tag web
proj list --tag rust

# Search within tagged projects
proj search app --tag web
```

### Integration with Other Tools

```bash
# Open in VSCode
code $(proj path my-project)

# Run tests in a project
(cd $(proj path my-project) && cargo test)

# Git operations
git -C $(proj path my-project) status

# Open GitHub repo
gh repo view --repo $(proj path my-project)
```

## Exit Codes

- `0` - Success
- `1` - General error
- `2` - Project not found
- `3` - Invalid configuration
- `4` - Shell integration not loaded (for `proj cd`)
