#!/usr/bin/env bash
set -euo pipefail

# Get the repository root directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"

# Prompt for sudo password upfront
echo "This script requires administrative privileges to install man pages."
sudo -v

# Create man page directories
echo "Creating man page directories..."
sudo mkdir -p /usr/local/share/man/man1 /usr/local/share/man/man7

# Copy man pages
echo "Installing man pages..."
sudo cp "$REPO_DIR/man/proj.1" /usr/local/share/man/man1/
sudo cp "$REPO_DIR/man/youssef-tools.7" /usr/local/share/man/man7/

# Rebuild man database
echo "Rebuilding man database..."
sudo mandb

echo ""
echo "✅ Man pages installed successfully!"
echo "   - proj(1) → /usr/local/share/man/man1/"
echo "   - youssef-tools(7) → /usr/local/share/man/man7/"
echo ""
echo "Test with: man proj or man youssef-tools"
