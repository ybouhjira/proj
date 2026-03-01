# GitHub Actions Workflows

This directory contains CI/CD workflows for the `proj` CLI tool.

## Workflows

### 1. CI (`ci.yml`)

Runs on every push and pull request to `main` and `develop` branches.

**Jobs:**
- **check**: Runs code quality checks
  - `cargo fmt --check` - Ensures code is formatted
  - `cargo clippy` - Lints code with strict warnings
  - `cargo test` - Runs all tests

- **build**: Builds release binaries on all platforms
  - Ubuntu, macOS, Windows
  - Uploads artifacts for each platform

**Caching:** Uses GitHub Actions cache for cargo registry, git index, and build artifacts to speed up builds.

### 2. Release (`release.yml`)

Triggers when a version tag is pushed (e.g., `v0.1.0`, `v1.2.3`).

**Platforms:**
- Linux x86_64 (`x86_64-unknown-linux-gnu`)
- Linux ARM64 (`aarch64-unknown-linux-gnu`) - built with `cross`
- macOS Intel (`x86_64-apple-darwin`)
- macOS Apple Silicon (`aarch64-apple-darwin`)
- Windows x86_64 (`x86_64-pc-windows-msvc`)

**Output:**
- Creates a GitHub release with auto-generated release notes
- Attaches binary archives:
  - Unix/macOS: `proj-{target}.tar.gz`
  - Windows: `proj-{target}.zip`

**How to trigger:**
```bash
# Create and push a tag
git tag v0.1.0
git push origin v0.1.0
```

### 3. Docs (`docs.yml`)

Deploys the Docusaurus documentation site to GitHub Pages.

**Triggers:**
- Push to `main` branch
- Only when files in `docs/` change

**Requirements:**
- GitHub Pages must be enabled in repository settings
- Source should be set to "GitHub Actions"

**Output:**
- Builds Docusaurus site from `docs/` directory
- Deploys to `https://<username>.github.io/<repo>/`

## Setup Requirements

### For Release Workflow
No additional setup needed. The workflow uses `GITHUB_TOKEN` which is automatically provided.

### For Docs Workflow
1. Go to repository Settings → Pages
2. Set Source to "GitHub Actions"
3. The workflow will automatically deploy on the next push to `main`

### For Cross-Compilation
The `aarch64-unknown-linux-gnu` target uses `cross` for cross-compilation. This is automatically installed during the workflow run.

## Local Testing

Before pushing, you can test locally:

```bash
# Format check
cargo fmt --all -- --check

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Tests
cargo test --all-features

# Build release
cargo build --release

# Build for specific target
cargo build --release --target x86_64-unknown-linux-gnu
```

## Troubleshooting

### CI failing on clippy warnings
Fix all clippy warnings locally before pushing:
```bash
cargo clippy --all-targets --all-features --fix
```

### Release not creating binaries
- Ensure tag follows semantic versioning: `vX.Y.Z`
- Check that the tag was pushed to GitHub
- Review workflow logs in Actions tab

### Docs not deploying
- Verify GitHub Pages is enabled with "GitHub Actions" source
- Check that changes were pushed to `main` branch
- Ensure `docs/` directory contains valid Docusaurus site
