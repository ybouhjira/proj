---
sidebar_position: 5
---

# AI Code Checks

:::info Coming Soon
This feature is currently in development. See [GitHub Issues](https://github.com/ybouhjira/proj/issues) for progress.
:::

## Overview

`proj` will integrate with Claude to provide intelligent code quality checks directly from your terminal. Get instant feedback on code quality, potential bugs, and best practices before committing.

## Planned Features

### Pre-Commit Checks

Automatically check code quality before committing:

```bash
# Enable in configuration
proj config set ai.check_on_commit true

# Manual check
proj check
```

### Smart Code Review

Get AI-powered code review feedback:

```bash
# Check specific files
proj check src/main.rs

# Check entire project
proj check .

# Check staged files only
proj check --staged
```

### Custom Rules

Define project-specific code quality rules:

```toml
[ai.rules]
# Enforce specific patterns
enforce_error_handling = true
check_naming_conventions = true
detect_code_smells = true

# Custom rules
custom = [
  "All public functions must have documentation",
  "Prefer async/await over raw promises",
  "Use proper error types, not strings"
]
```

## Configuration

Once available, configure AI checks in your `~/.config/proj/config.toml`:

```toml
[ai]
# Enable AI code quality checks
enabled = true

# Claude model to use
model = "claude-3-5-sonnet"

# API key from environment variable
api_key_env = "ANTHROPIC_API_KEY"

# Run checks on git commit
check_on_commit = true

# Files to check
include = ["*.rs", "*.ts", "*.tsx", "*.py", "*.go"]
exclude = [
  "target/",
  "node_modules/",
  "*.test.*",
  "*.spec.*",
  "dist/",
  "build/"
]

# Check severity (error, warning, info)
min_severity = "warning"
```

## Planned Commands

### `proj check`

Run code quality checks:

```bash
# Check current directory
proj check

# Check specific files
proj check src/main.rs lib/utils.ts

# Check with specific model
proj check --model claude-3-5-sonnet

# Show only errors
proj check --severity error

# Output as JSON
proj check --json
```

### `proj check --watch`

Continuously check files as they change:

```bash
# Watch mode
proj check --watch

# Watch specific directory
proj check --watch src/
```

### `proj check --fix`

Automatically fix issues where possible:

```bash
# Fix all auto-fixable issues
proj check --fix

# Preview fixes without applying
proj check --fix --dry-run
```

## Integration with Git Hooks

Integrate with pre-commit hooks:

```bash
# Install git hook
proj check install-hook

# Uninstall
proj check uninstall-hook
```

The hook will run before each commit and prevent commits if critical issues are found.

## Check Categories

The AI checker will analyze:

### Code Quality
- Code smells and anti-patterns
- Dead code detection
- Complexity analysis
- Duplication detection

### Security
- Potential security vulnerabilities
- Unsafe operations
- Input validation issues
- Dependency vulnerabilities

### Best Practices
- Language-specific best practices
- Framework conventions
- Design patterns
- Performance optimization opportunities

### Documentation
- Missing documentation
- Outdated comments
- TODO/FIXME tracking
- API documentation completeness

## Example Output

```bash
$ proj check src/main.rs

Checking src/main.rs...

❌ Error: Potential null pointer dereference
  Line 45: `user.name.len()`
  Suggestion: Check if `user.name` is Some before accessing

⚠️  Warning: Inefficient string concatenation in loop
  Line 78-82: Using + operator in loop
  Suggestion: Use a String buffer for better performance

ℹ️  Info: Consider adding error context
  Line 120: `File::open(path)?`
  Suggestion: Add context with .context() or .map_err()

✅ 1 error, 1 warning, 1 info
```

## Privacy & Security

- Code is sent to Claude API for analysis
- No code is stored or logged by Anthropic
- API key is stored securely in environment variables
- You can disable checks at any time

## Roadmap

Track development progress:

1. **Phase 1**: Basic file checking with Claude API ✨ In Progress
2. **Phase 2**: Git integration and pre-commit hooks
3. **Phase 3**: Custom rules engine
4. **Phase 4**: Auto-fix capabilities
5. **Phase 5**: Continuous monitoring and watch mode

See [GitHub Issues](https://github.com/ybouhjira/proj/issues) for detailed roadmap and contribute to development.

## Getting Involved

Want to help build this feature?

- 📝 [Feature Requests](https://github.com/ybouhjira/proj/issues/new?labels=enhancement)
- 🐛 [Report Bugs](https://github.com/ybouhjira/proj/issues/new?labels=bug)
- 💬 [Discussions](https://github.com/ybouhjira/proj/discussions)
- 🤝 [Contributing Guide](https://github.com/ybouhjira/proj/blob/main/CONTRIBUTING.md)

## Next Steps

While waiting for this feature:
- Set up your [shell integration](./shell-integration.md)
- Explore all available [commands](./commands.md)
- Customize your [configuration](./configuration.md)
