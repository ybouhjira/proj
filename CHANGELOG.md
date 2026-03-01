# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Unified project dashboard showing local and remote repositories
- Fuzzy search navigation with `proj cd <query>`
- Sync status dashboard tracking dirty repos, untracked changes, and sync state
- GitHub integration for cloning and creating repositories
- Quality check system for running linters and tests across projects
- Shell integration for instant directory navigation
- Rich metadata display (language, stars, last push time, dirty file count)
- Support for local-only and remote-only filtering
- Configuration system via `~/.config/proj/config.toml`
- Exclude patterns for project discovery
- Detailed project info command

### Features by Command
- `ls` - List all projects with status indicators
- `cd` - Fuzzy find and navigate to projects
- `sync` - Show what needs attention (dirty, ahead, behind)
- `clone` - Clone GitHub repos to projects directory
- `new` - Create new local + GitHub projects
- `open` - Open projects in browser or file manager
- `info` - Show detailed project information
- `check` - Run quality checks (linters, tests)
- `init` - Generate shell integration scripts

## [0.1.0] - Unreleased

Initial development version.

[Unreleased]: https://github.com/ybouhjira/proj/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ybouhjira/proj/releases/tag/v0.1.0
