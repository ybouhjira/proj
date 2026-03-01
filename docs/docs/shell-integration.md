---
sidebar_position: 4
---

# Shell Integration

Enable seamless navigation between projects with shell integration.

## Overview

Shell integration allows you to use `proj cd` to change directories directly, without having to use subshells or `cd $(proj path ...)`.

## Setup

### Bash

Add this line to your `~/.bashrc`:

```bash
eval "$(proj init bash)"
```

Then reload your shell:

```bash
source ~/.bashrc
```

---

### Zsh

Add this line to your `~/.zshrc`:

```bash
eval "$(proj init zsh)"
```

Then reload your shell:

```bash
source ~/.zshrc
```

---

### Fish

Add this line to your `~/.config/fish/config.fish`:

```fish
proj init fish | source
```

Then reload your shell:

```fish
source ~/.config/fish/config.fish
```

## How It Works

The shell integration:

1. Defines a shell function that wraps the `proj` command
2. Intercepts the `proj cd` command
3. Executes the `cd` command in your current shell
4. Passes all other commands to the `proj` binary

## Usage

Once integrated, you can navigate seamlessly:

```bash
# Change to a project directory
proj cd my-project

# Fuzzy matching works
proj cd proj  # Matches "my-project", "proj-cli", etc.

# All other commands work normally
proj list
proj add .
```

## Features

### Fuzzy Matching

The `cd` command supports fuzzy matching:

```bash
# If you have projects: my-awesome-project, awesome-tool, project-manager
proj cd awes  # Matches "my-awesome-project" or "awesome-tool"
```

If multiple matches are found, you'll be prompted to choose.

### Tab Completion

Tab completion is supported for project names:

```bash
proj cd my-<TAB>
# Completes to: proj cd my-project
```

### Directory History

The integration works with shell directory history:

```bash
# Navigate to project
proj cd my-project

# Use cd history to go back
cd -
```

### Subshells

Shell integration works in subshells:

```bash
# Works in command substitution
echo "Currently in: $(pwd)"
proj cd my-project
echo "Now in: $(pwd)"
```

## Advanced Configuration

### Custom Aliases

Add custom aliases for common workflows:

```bash
# In ~/.bashrc or ~/.zshrc

# Quick project navigation
alias p='proj cd'

# List and search
alias pl='proj list'
alias ps='proj search'

# Quick add
alias pa='proj add .'
```

### Integration with Other Tools

Combine with other tools for powerful workflows:

#### fzf Integration

Use [fzf](https://github.com/junegunn/fzf) for interactive project selection:

```bash
# Add to ~/.bashrc or ~/.zshrc
pcd() {
  local project=$(proj list --format json | jq -r '.[].name' | fzf)
  if [ -n "$project" ]; then
    proj cd "$project"
  fi
}
```

#### z/zoxide Integration

Works alongside [z](https://github.com/rupa/z) or [zoxide](https://github.com/ajeetdsouza/zoxide):

```bash
# Use proj for project management
proj cd my-project

# z/zoxide still work for frecent directories
z my-pr
```

## Troubleshooting

### Shell Integration Not Working

1. **Check if integration is loaded:**

```bash
# Should show the wrapper function
type proj
```

2. **Verify init command works:**

```bash
proj init bash  # or zsh, fish
```

3. **Reload shell configuration:**

```bash
# Bash
source ~/.bashrc

# Zsh
source ~/.zshrc

# Fish
source ~/.config/fish/config.fish
```

### `proj cd` Command Not Found

Make sure you've added the integration to your shell config and reloaded it.

### Conflicts with Existing Functions

If you have an existing `proj` function or alias:

```bash
# Check for conflicts
type proj

# Remove existing alias
unalias proj

# Or rename the integration
eval "$(proj init bash)" && alias p='proj'
```

### Permissions Issues

If you get permission errors:

```bash
# Ensure proj binary is executable
chmod +x $(which proj)

# Check config file permissions
ls -la ~/.config/proj/config.toml
```

## Manual Integration

If you prefer to write your own integration:

### Bash/Zsh Manual Integration

```bash
proj() {
  if [ "$1" = "cd" ]; then
    local project_path=$(command proj path "$2")
    if [ -n "$project_path" ]; then
      cd "$project_path"
    else
      echo "Project not found: $2" >&2
      return 1
    fi
  else
    command proj "$@"
  fi
}
```

### Fish Manual Integration

```fish
function proj
    if test "$argv[1]" = "cd"
        set -l project_path (command proj path $argv[2])
        if test -n "$project_path"
            cd $project_path
        else
            echo "Project not found: $argv[2]" >&2
            return 1
        end
    else
        command proj $argv
    end
end
```

## Uninstalling Integration

To remove shell integration:

1. Remove the `eval "$(proj init ...)"` line from your shell config
2. Reload your shell or start a new terminal session

The `proj` binary will still work, but `proj cd` will no longer change directories.

## Next Steps

- Learn about all [commands](./commands.md)
- Customize your [configuration](./configuration.md)
- Explore [AI code checks](./ai-checks.md) (coming soon)
