# Contributing to proj

Thanks for your interest in contributing to `proj`! This guide will help you get started.

## Development Setup

1. **Fork and clone**:
   ```bash
   gh repo fork ybouhjira/proj --clone
   cd proj
   ```

2. **Build the project**:
   ```bash
   cargo build
   ```

3. **Run tests**:
   ```bash
   cargo test
   ```

## Making Changes

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following these guidelines:
   - Run `cargo fmt` before committing (formats code)
   - Run `cargo clippy` to catch common mistakes
   - Add tests for new functionality
   - Update documentation if needed

3. **Test your changes**:
   ```bash
   cargo test
   cargo run -- ls --local  # Test CLI commands
   ```

4. **Commit with conventional commits**:
   ```bash
   git commit -m "feat: add new feature"
   git commit -m "fix: resolve bug in sync command"
   git commit -m "docs: update README examples"
   ```

## Submitting a Pull Request

1. **Push your branch**:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Open a PR** with:
   - Clear title describing the change
   - Description of what changed and why
   - Reference any related issues

3. **Respond to feedback** — maintainers may request changes

## Code Style

- Follow Rust naming conventions
- Use `cargo fmt` for formatting (enforced by CI)
- Run `cargo clippy` to catch issues
- Write clear commit messages (see conventional commits above)

## Issue Labels

- `bug` — Something isn't working
- `enhancement` — New feature request
- `good first issue` — Good for newcomers
- `help wanted` — Extra attention needed
- `documentation` — Docs improvements

## Questions?

Open an issue or start a discussion on GitHub!
