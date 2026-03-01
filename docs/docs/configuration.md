---
sidebar_position: 3
---

# Configuration

Learn how to customize `proj` to fit your workflow.

## Configuration File Location

The configuration file is automatically created at:

- **Linux/macOS**: `~/.config/proj/config.toml`
- **Windows**: `%APPDATA%\proj\config.toml`

You can specify a custom location with the `--config` flag:

```bash
proj --config /path/to/config.toml list
```

## Configuration Format

The configuration file uses [TOML](https://toml.io/) format:

```toml
[projects]
root = "~/code"
auto_discover = true

[shell]
default = "zsh"

[display]
color = true
format = "table"

[ai]
enabled = false
model = "claude-3-5-sonnet"
api_key_env = "ANTHROPIC_API_KEY"
```

## Configuration Sections

### `[projects]`

Project management settings.

```toml
[projects]
# Default directory for cloning git repositories
root = "~/code"

# Automatically discover projects in root directory
auto_discover = false

# Ignore patterns for auto-discovery
ignore = [".git", "node_modules", "target"]
```

**Options:**

- `root` (string) - Default directory for new projects
- `auto_discover` (boolean) - Auto-discover git repos in root
- `ignore` (array) - Patterns to ignore during discovery

---

### `[shell]`

Shell integration settings.

```toml
[shell]
# Default shell (bash, zsh, fish)
default = "zsh"

# Custom prompt when in project directory
prompt_format = "{name} [{path}]"
```

**Options:**

- `default` (string) - Default shell type
- `prompt_format` (string) - Custom prompt format (coming soon)

---

### `[display]`

Output and display settings.

```toml
[display]
# Enable colored output
color = true

# Output format (table, list, json)
format = "table"

# Show full paths
full_paths = false

# Date format
date_format = "%Y-%m-%d %H:%M"
```

**Options:**

- `color` (boolean) - Enable/disable colored output
- `format` (string) - Default output format (`table`, `list`, `json`)
- `full_paths` (boolean) - Show full or relative paths
- `date_format` (string) - Date format string

---

### `[ai]` (Coming Soon)

Code quality check settings.

```toml
[ai]
# Enable AI code quality checks
enabled = false

# Claude model to use
model = "claude-3-5-sonnet"

# Environment variable containing API key
api_key_env = "ANTHROPIC_API_KEY"

# Check on commit
check_on_commit = false

# File patterns to check
include = ["*.rs", "*.ts", "*.py"]
exclude = ["target/", "node_modules/", "*.test.*"]
```

**Options:**

- `enabled` (boolean) - Enable AI checks
- `model` (string) - Claude model name
- `api_key_env` (string) - Environment variable for API key
- `check_on_commit` (boolean) - Run checks on git commit
- `include` (array) - File patterns to include
- `exclude` (array) - File patterns to exclude

---

## Managing Configuration

### View Current Configuration

```bash
proj config show
```

### Edit Configuration File

Opens the config file in your default editor:

```bash
proj config edit
```

### Get/Set Values

```bash
# Get a value
proj config get projects.root

# Set a value
proj config set projects.root ~/code

# Set nested values
proj config set display.color true
```

## Environment Variables

Override configuration with environment variables:

- `PROJ_CONFIG` - Custom config file path
- `PROJ_ROOT` - Override projects.root
- `PROJ_NO_COLOR` - Disable colors (set to `1`)
- `ANTHROPIC_API_KEY` - Claude API key (for AI checks)

```bash
# Use custom config
PROJ_CONFIG=~/my-config.toml proj list

# Disable colors
PROJ_NO_COLOR=1 proj list

# Override root directory
PROJ_ROOT=~/projects proj add .
```

## Example Configurations

### Minimal Configuration

```toml
[projects]
root = "~/code"
```

### Development Setup

```toml
[projects]
root = "~/code"
auto_discover = true
ignore = [".git", "node_modules", "target", "build", "dist"]

[shell]
default = "zsh"

[display]
color = true
format = "table"
full_paths = false
```

### With AI Checks (Coming Soon)

```toml
[projects]
root = "~/code"

[ai]
enabled = true
model = "claude-3-5-sonnet"
api_key_env = "ANTHROPIC_API_KEY"
check_on_commit = true
include = ["*.rs", "*.ts", "*.tsx", "*.py"]
exclude = [
  "target/",
  "node_modules/",
  "*.test.*",
  "*.spec.*",
  "dist/"
]
```

## Configuration Validation

Validate your configuration file:

```bash
proj config validate
```

This will check:
- TOML syntax is valid
- All required fields are present
- Values are of correct type
- Paths exist and are accessible

## Migrating Configuration

When upgrading `proj`, your configuration is automatically migrated. A backup is created at:

```
~/.config/proj/config.toml.backup.<timestamp>
```

To manually migrate:

```bash
proj config migrate
```

## Troubleshooting

### Configuration Not Loading

```bash
# Check config file exists
ls ~/.config/proj/config.toml

# Validate configuration
proj config validate

# Show effective configuration
proj config show
```

### Reset to Defaults

```bash
# Backup current config
cp ~/.config/proj/config.toml ~/.config/proj/config.toml.backup

# Delete config (will be recreated with defaults)
rm ~/.config/proj/config.toml

# Run any command to recreate
proj list
```

## Next Steps

- Explore all [commands](./commands.md)
- Set up [shell integration](./shell-integration.md)
- Learn about [AI code checks](./ai-checks.md) (coming soon)
