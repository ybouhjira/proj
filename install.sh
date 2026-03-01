#!/usr/bin/env bash
# proj installer script
# https://github.com/ybouhjira/proj
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/ybouhjira/proj/main/install.sh | sh
#   ./install.sh
#   ./install.sh --no-shell --no-completions

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Flags
SKIP_SHELL=false
SKIP_COMPLETIONS=false

# Parse arguments
for arg in "$@"; do
    case $arg in
        --no-shell)
            SKIP_SHELL=true
            shift
            ;;
        --no-completions)
            SKIP_COMPLETIONS=true
            shift
            ;;
    esac
done

echo_step() {
    echo -e "${BLUE}==>${NC} $1"
}

echo_success() {
    echo -e "${GREEN}✓${NC} $1"
}

echo_error() {
    echo -e "${RED}✗${NC} $1" >&2
}

echo_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Detect OS and architecture
detect_platform() {
    local os
    local arch
    local target

    # Detect OS
    case "$(uname -s)" in
        Linux*)     os=linux ;;
        Darwin*)    os=darwin ;;
        MINGW*|MSYS*|CYGWIN*) os=windows ;;
        *)
            echo_error "Unsupported OS: $(uname -s)"
            exit 1
            ;;
    esac

    # Detect architecture
    case "$(uname -m)" in
        x86_64|amd64)   arch=x86_64 ;;
        aarch64|arm64)  arch=aarch64 ;;
        *)
            echo_error "Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac

    # Build target triple
    case "$os-$arch" in
        linux-x86_64)       target="x86_64-unknown-linux-gnu" ;;
        linux-aarch64)      target="aarch64-unknown-linux-gnu" ;;
        darwin-x86_64)      target="x86_64-apple-darwin" ;;
        darwin-aarch64)     target="aarch64-apple-darwin" ;;
        windows-x86_64)     target="x86_64-pc-windows-msvc" ;;
        *)
            echo_error "Unsupported platform: $os-$arch"
            exit 1
            ;;
    esac

    echo "$target"
}

# Detect if curl or wget is available
download() {
    local url=$1
    local output=$2

    if command -v curl &>/dev/null; then
        curl -fsSL "$url" -o "$output"
    elif command -v wget &>/dev/null; then
        wget -q "$url" -O "$output"
    else
        echo_error "Neither curl nor wget found. Please install one of them."
        exit 1
    fi
}

# Check if proj is already installed
check_existing() {
    if command -v proj &>/dev/null; then
        local version
        version=$(proj --version 2>/dev/null | awk '{print $2}' || echo "unknown")
        echo_warning "proj is already installed (version: $version)"
        echo "This script will update it to the latest version."
        echo ""
    fi
}

# Determine install directory
get_install_dir() {
    if [ -d "$HOME/.cargo/bin" ]; then
        echo "$HOME/.cargo/bin"
    else
        echo "$HOME/.local/bin"
    fi
}

# Download and install binary
install_binary() {
    local target=$1
    local install_dir=$2
    local tmpdir
    local url

    tmpdir=$(mktemp -d)
    trap "rm -rf $tmpdir" EXIT

    url="https://github.com/ybouhjira/proj/releases/latest/download/proj-${target}.tar.gz"

    echo_step "Downloading proj for $target..."
    download "$url" "$tmpdir/proj.tar.gz"

    echo_step "Extracting binary..."
    tar -xzf "$tmpdir/proj.tar.gz" -C "$tmpdir"

    echo_step "Installing to $install_dir/proj..."
    mkdir -p "$install_dir"

    if [ -f "$tmpdir/proj" ]; then
        cp "$tmpdir/proj" "$install_dir/proj"
        chmod +x "$install_dir/proj"
    else
        echo_error "Binary not found in archive"
        exit 1
    fi

    echo_success "Binary installed to $install_dir/proj"
}

# Detect shell
detect_shell() {
    if [ -n "${ZSH_VERSION:-}" ]; then
        echo "zsh"
    elif [ -n "${BASH_VERSION:-}" ]; then
        echo "bash"
    elif [ -n "${FISH_VERSION:-}" ]; then
        echo "fish"
    else
        # Fallback to $SHELL
        basename "$SHELL"
    fi
}

# Install oh-my-zsh plugin
install_omz_plugin() {
    local omz_custom="${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}"
    local plugin_dir="$omz_custom/plugins/proj"
    local zshrc="$HOME/.zshrc"

    if [ ! -d "$HOME/.oh-my-zsh" ]; then
        return 1
    fi

    echo_step "Installing oh-my-zsh plugin..."
    mkdir -p "$plugin_dir"

    # Download plugin file
    local tmpdir
    tmpdir=$(mktemp -d)
    trap "rm -rf $tmpdir" EXIT

    download "https://raw.githubusercontent.com/ybouhjira/proj/main/plugins/oh-my-zsh/proj.plugin.zsh" "$tmpdir/proj.plugin.zsh"
    cp "$tmpdir/proj.plugin.zsh" "$plugin_dir/proj.plugin.zsh"

    # Add to plugins list if not already there
    if ! grep -q "plugins=.*proj" "$zshrc" 2>/dev/null; then
        echo_step "Adding proj to oh-my-zsh plugins..."
        # Backup zshrc
        cp "$zshrc" "$zshrc.backup"

        # Add proj to plugins array
        if grep -q "^plugins=(" "$zshrc"; then
            sed -i.tmp 's/^plugins=(\(.*\))/plugins=(\1 proj)/' "$zshrc"
            rm -f "$zshrc.tmp"
            echo_success "Added proj to plugins in ~/.zshrc"
        else
            echo_warning "Could not automatically add proj to plugins. Add it manually:"
            echo "  plugins=(... proj)"
        fi
    fi

    echo_success "oh-my-zsh plugin installed"
    return 0
}

# Install shell wrapper for plain zsh
install_zsh_wrapper() {
    local zshrc="$HOME/.zshrc"

    # Check if already installed
    if grep -q "# proj shell wrapper" "$zshrc" 2>/dev/null; then
        echo_success "Shell wrapper already in ~/.zshrc"
        return
    fi

    echo_step "Adding shell wrapper to ~/.zshrc..."
    cat >> "$zshrc" << 'EOF'

# proj shell wrapper
proj() {
    if [ "$1" = "cd" ]; then
        shift
        local dir
        dir=$(command proj cd "$@" 2>&1)
        if [ $? -eq 0 ] && [ -n "$dir" ] && [ -d "$dir" ]; then
            builtin cd "$dir"
        else
            echo "$dir" >&2
        fi
    else
        command proj "$@"
    fi
}
EOF

    echo_success "Shell wrapper added to ~/.zshrc"
}

# Install shell wrapper for bash
install_bash_wrapper() {
    local bashrc="$HOME/.bashrc"

    # Check if already installed
    if grep -q "# proj shell wrapper" "$bashrc" 2>/dev/null; then
        echo_success "Shell wrapper already in ~/.bashrc"
        return
    fi

    echo_step "Adding shell wrapper to ~/.bashrc..."
    cat >> "$bashrc" << 'EOF'

# proj shell wrapper
proj() {
    if [ "$1" = "cd" ]; then
        shift
        local dir
        dir=$(command proj cd "$@" 2>&1)
        if [ $? -eq 0 ] && [ -n "$dir" ] && [ -d "$dir" ]; then
            builtin cd "$dir"
        else
            echo "$dir" >&2
        fi
    else
        command proj "$@"
    fi
}
EOF

    echo_success "Shell wrapper added to ~/.bashrc"
}

# Install shell wrapper for fish
install_fish_wrapper() {
    local fish_functions="$HOME/.config/fish/functions"
    local fish_file="$fish_functions/proj.fish"

    mkdir -p "$fish_functions"

    echo_step "Creating fish function at $fish_file..."
    cat > "$fish_file" << 'EOF'
function proj
    if test "$argv[1]" = "cd"
        set -e argv[1]
        set dir (command proj cd $argv 2>&1)
        if test $status -eq 0; and test -n "$dir"; and test -d "$dir"
            builtin cd "$dir"
        else
            echo "$dir" >&2
        end
    else
        command proj $argv
    end
end
EOF

    echo_success "Fish function created at $fish_file"
}

# Install shell integration
install_shell_integration() {
    local shell
    shell=$(detect_shell)

    case "$shell" in
        zsh)
            if install_omz_plugin; then
                return
            else
                install_zsh_wrapper
            fi
            ;;
        bash)
            install_bash_wrapper
            ;;
        fish)
            install_fish_wrapper
            ;;
        *)
            echo_warning "Unknown shell: $shell"
            echo "You'll need to manually set up the shell wrapper."
            ;;
    esac
}

# Install completions
install_completions() {
    local shell
    shell=$(detect_shell)

    case "$shell" in
        zsh)
            local comp_dir="$HOME/.zsh/completions"
            mkdir -p "$comp_dir"
            echo_step "Installing zsh completions..."
            proj completions zsh > "$comp_dir/_proj" 2>/dev/null || true
            echo_success "zsh completions installed to $comp_dir/_proj"
            echo_warning "You may need to add this to your ~/.zshrc:"
            echo "  fpath=($comp_dir \$fpath)"
            echo "  autoload -Uz compinit && compinit"
            ;;
        bash)
            local comp_dir="$HOME/.bash_completions"
            mkdir -p "$comp_dir"
            echo_step "Installing bash completions..."
            proj completions bash > "$comp_dir/proj" 2>/dev/null || true
            echo_success "bash completions installed to $comp_dir/proj"
            echo_warning "You may need to add this to your ~/.bashrc:"
            echo "  source $comp_dir/proj"
            ;;
        fish)
            local comp_dir="$HOME/.config/fish/completions"
            mkdir -p "$comp_dir"
            echo_step "Installing fish completions..."
            proj completions fish > "$comp_dir/proj.fish" 2>/dev/null || true
            echo_success "fish completions installed to $comp_dir/proj.fish"
            ;;
    esac
}

# Install man pages (optional)
install_man_pages() {
    local man_dir="/usr/local/share/man/man1"

    if [ ! -d "/usr/local/share/man" ]; then
        return
    fi

    if [ ! -w "/usr/local/share/man" ] && [ ! -w "$man_dir" ]; then
        # Try user man directory instead
        man_dir="$HOME/.local/share/man/man1"
        mkdir -p "$man_dir"
    fi

    echo_step "Installing man pages..."
    local tmpdir
    tmpdir=$(mktemp -d)
    trap "rm -rf $tmpdir" EXIT

    # Download man pages from repo
    for page in proj.1 proj-cd.1 proj-check.1 proj-info.1 proj-ls.1 proj-new.1 proj-open.1 proj-sync.1; do
        if download "https://raw.githubusercontent.com/ybouhjira/proj/main/man/$page" "$tmpdir/$page" 2>/dev/null; then
            cp "$tmpdir/$page" "$man_dir/$page" 2>/dev/null || true
        fi
    done

    echo_success "Man pages installed to $man_dir"
}

# Main installation flow
main() {
    echo ""
    echo "╔═══════════════════════════════════════╗"
    echo "║   proj - Project Management CLI      ║"
    echo "║   Installation Script                ║"
    echo "╚═══════════════════════════════════════╝"
    echo ""

    check_existing

    local target
    target=$(detect_platform)
    echo_step "Detected platform: $target"

    local install_dir
    install_dir=$(get_install_dir)

    # Install binary
    install_binary "$target" "$install_dir"

    # Ensure install_dir is in PATH
    if ! echo "$PATH" | grep -q "$install_dir"; then
        echo_warning "$install_dir is not in your PATH"
        echo "Add this to your shell config:"
        echo "  export PATH=\"$install_dir:\$PATH\""
    fi

    # Install shell integration
    if [ "$SKIP_SHELL" = false ]; then
        install_shell_integration
    else
        echo_warning "Skipping shell integration (--no-shell)"
    fi

    # Install completions
    if [ "$SKIP_COMPLETIONS" = false ]; then
        install_completions
    else
        echo_warning "Skipping completions (--no-completions)"
    fi

    # Install man pages (best effort)
    install_man_pages 2>/dev/null || true

    echo ""
    echo "╔═══════════════════════════════════════╗"
    echo "║   Installation Complete!              ║"
    echo "╚═══════════════════════════════════════╝"
    echo ""
    echo_success "proj installed successfully!"
    echo ""
    echo "Installed components:"
    echo "  • Binary: $install_dir/proj"
    if [ "$SKIP_SHELL" = false ]; then
        echo "  • Shell integration: $(detect_shell)"
    fi
    if [ "$SKIP_COMPLETIONS" = false ]; then
        echo "  • Shell completions: $(detect_shell)"
    fi
    echo ""
    echo "Next steps:"
    echo "  1. Restart your shell or run: source ~/.$(detect_shell)rc"
    echo "  2. Initialize your first project: proj new my-project"
    echo "  3. List projects: proj ls"
    echo "  4. Jump to a project: proj cd my-project"
    echo ""
    echo "Documentation: https://github.com/ybouhjira/proj"
    echo ""
}

main
