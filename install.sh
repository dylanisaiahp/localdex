#!/usr/bin/env bash
# install.sh — Install or update ldx (localdex)
# Supports: Linux, macOS, Windows (Git Bash)
# Usage: ./install.sh [options]
#
# curl install (once releases are available):
#   curl -sSf https://raw.githubusercontent.com/dylanisaiahp/localdex/main/install.sh | bash

set -e

# ─── Colors ───────────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
RESET='\033[0m'

REPO_URL="https://github.com/dylanisaiahp/localdex"
REPO_API="https://api.github.com/repos/dylanisaiahp/localdex/releases/latest"
REPO_NAME="dylanisaiahp/localdex"

# ─── Detect OS ────────────────────────────────────────────────────────────────
detect_os() {
    case "$(uname -s)" in
        Linux*)   echo "linux" ;;
        Darwin*)  echo "macos" ;;
        MINGW*|MSYS*|CYGWIN*) echo "windows" ;;
        *)        echo "unknown" ;;
    esac
}

OS=$(detect_os)

if [ "$OS" = "unknown" ]; then
    echo -e "${RED}Unsupported OS. Exiting.${RESET}"
    exit 1
fi

# ─── Binary names ─────────────────────────────────────────────────────────────
if [ "$OS" = "windows" ]; then
    BINARY_NAME="localdex.exe"
    ALIAS_NAME="ldx.exe"
else
    BINARY_NAME="localdex"
    ALIAS_NAME="ldx"
fi

# ─── Parse arguments ──────────────────────────────────────────────────────────
FORCE=false
FROM_SOURCE=false
FROM_BINARY=false
KEEP_SOURCE=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --force)
            FORCE=true
            shift ;;
        --from-source)
            FROM_SOURCE=true
            shift ;;
        --binary)
            FROM_BINARY=true
            shift ;;
        --keep-source)
            KEEP_SOURCE=true
            shift ;;
        --help)
            echo "Usage: ./install.sh [options]"
            echo ""
            echo "Options:"
            echo "  --force          Force reinstall even if up to date"
            echo "  --from-source    Build from source (default)"
            echo "  --binary         Download pre-built binary (when available)"
            echo "  --keep-source    Keep cloned source folder after install"
            echo "  --help           Show this help message"
            exit 0 ;;
        *)
            echo -e "${RED}Unknown argument: $1${RESET}"
            echo "Run ./install.sh --help for usage"
            exit 1 ;;
    esac
done

# ─── Header ───────────────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}${BOLD}🔍 ldx — Installer${RESET}"
echo -e "${CYAN}OS: $OS${RESET}"
echo ""

# ─── Get latest version from GitHub ──────────────────────────────────────────
get_latest_version() {
    if command -v curl &> /dev/null; then
        curl -s "$REPO_API" 2>/dev/null | grep '"tag_name"' | head -1 | grep -oP 'v[\d.]+' | head -1
    elif command -v wget &> /dev/null; then
        wget -qO- "$REPO_API" 2>/dev/null | grep '"tag_name"' | head -1 | grep -oP 'v[\d.]+' | head -1
    else
        echo ""
    fi
}

# ─── Get installed version ────────────────────────────────────────────────────
get_installed_version() {
    if command -v ldx &> /dev/null; then
        ldx --version 2>/dev/null | grep -oP 'v[\d.]+' | head -1
    else
        echo ""
    fi
}

# ─── Check if source files are present ───────────────────────────────────────
is_in_repo() {
    [ -f "Cargo.toml" ] && grep -q "localdex" "Cargo.toml" 2>/dev/null
}

# ─── Destination picker ───────────────────────────────────────────────────────
pick_destination() {
    echo -e "${BOLD}Where would you like to install ldx?${RESET}"
    echo ""

    if [ "$OS" = "windows" ]; then
        CARGO_BIN="$USERPROFILE/.cargo/bin"
        USER_BIN="$USERPROFILE/bin"
        SYSTEM_BIN="/c/Program Files/ldx"
        echo -e "  ${CYAN}1)${RESET} $CARGO_BIN ${GREEN}(default, already in PATH)${RESET}"
        echo -e "  ${CYAN}2)${RESET} $USER_BIN"
        echo -e "  ${CYAN}3)${RESET} $SYSTEM_BIN ${YELLOW}(may require admin)${RESET}"
        echo -e "  ${CYAN}4)${RESET} Custom path"
    else
        CARGO_BIN="$HOME/.cargo/bin"
        USER_BIN="$HOME/.local/bin"
        SYSTEM_BIN="/usr/local/bin"
        echo -e "  ${CYAN}1)${RESET} $CARGO_BIN ${GREEN}(default, already in PATH)${RESET}"
        echo -e "  ${CYAN}2)${RESET} $USER_BIN"
        echo -e "  ${CYAN}3)${RESET} $SYSTEM_BIN ${YELLOW}(requires sudo)${RESET}"
        echo -e "  ${CYAN}4)${RESET} Custom path"
    fi

    echo ""
    read -rp "Choice [1-4] (default: 1): " CHOICE
    CHOICE=${CHOICE:-1}

    case "$CHOICE" in
        1) DEST="$CARGO_BIN"; SKIP_PATH=true ;;
        2) DEST="$USER_BIN";  SKIP_PATH=false ;;
        3) DEST="$SYSTEM_BIN"; SKIP_PATH=false ;;
        4)
            read -rp "Enter custom path: " DEST
            SKIP_PATH=false ;;
        *)
            echo -e "${RED}Invalid choice. Exiting.${RESET}"
            exit 1 ;;
    esac
}

# ─── PATH setup ───────────────────────────────────────────────────────────────
setup_path() {
    if [ "$SKIP_PATH" = true ]; then
        return
    fi

    if echo "$PATH" | grep -q "$DEST"; then
        echo -e "${GREEN}✓ $DEST is already in PATH${RESET}"
        return
    fi

    echo ""
    echo -e "${YELLOW}$DEST is not in your PATH.${RESET}"
    read -rp "Add it now? [y/N]: " ADD_PATH
    if [[ "$ADD_PATH" =~ ^[Yy]$ ]]; then
        if [ "$OS" = "windows" ]; then
            SHELL_CONFIG="$HOME/.bashrc"
        elif [ -n "$ZSH_VERSION" ] || [ "$SHELL" = "/bin/zsh" ] || [ "$SHELL" = "/usr/bin/zsh" ]; then
            SHELL_CONFIG="$HOME/.zshrc"
        elif [ -n "$BASH_VERSION" ]; then
            SHELL_CONFIG="$HOME/.bashrc"
        else
            SHELL_CONFIG="$HOME/.profile"
        fi

        echo "" >> "$SHELL_CONFIG"
        echo "# ldx" >> "$SHELL_CONFIG"
        echo "export PATH=\"\$PATH:$DEST\"" >> "$SHELL_CONFIG"
        echo -e "${GREEN}✓ Added to $SHELL_CONFIG${RESET}"
        echo -e "${YELLOW}Run: source $SHELL_CONFIG${RESET}"
    fi
}

# ─── Install from source ──────────────────────────────────────────────────────
install_from_source() {
    # Check for cargo
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}cargo not found.${RESET}"
        echo -e "Install Rust first: ${CYAN}https://rustup.rs${RESET}"
        exit 1
    fi

    # Clone if not already in repo
    if ! is_in_repo; then
        if ! command -v git &> /dev/null; then
            echo -e "${RED}git not found. Please install git first.${RESET}"
            exit 1
        fi
        echo -e "${CYAN}Cloning repository...${RESET}"
        git clone "$REPO_URL" localdex-src
        cd localdex-src
    fi

    echo -e "${CYAN}Building from source...${RESET}"
    echo ""
    cargo build --release
    echo ""
    echo -e "${GREEN}✓ Build successful${RESET}"

    pick_destination
    mkdir -p "$DEST"

    if [ "$CHOICE" = "3" ] && [ "$OS" != "windows" ]; then
        sudo cp "target/release/$BINARY_NAME" "$DEST/$BINARY_NAME"
        sudo cp "target/release/$BINARY_NAME" "$DEST/$ALIAS_NAME"
    else
        cp "target/release/$BINARY_NAME" "$DEST/$BINARY_NAME"
        cp "target/release/$BINARY_NAME" "$DEST/$ALIAS_NAME"
    fi

    echo -e "${GREEN}✓ Installed: $DEST/$BINARY_NAME${RESET}"
    echo -e "${GREEN}✓ Alias:     $DEST/$ALIAS_NAME${RESET}"

    setup_path

    # Clean up cloned dir if we cloned
    if [ -d "../localdex-src" ] || [ "$(basename "$PWD")" = "localdex-src" ]; then
        cd ..
        if [ "$KEEP_SOURCE" = true ]; then
            echo -e "${CYAN}✓ Source kept at: $(pwd)/localdex-src${RESET}"
        else
            rm -rf localdex-src
            echo -e "${GREEN}✓ Cleaned up source${RESET}"
        fi
    fi
}

# ─── Install from binary ──────────────────────────────────────────────────────
install_from_binary() {
    echo -e "${YELLOW}Pre-built binaries are not yet available for this version.${RESET}"
    echo -e "${CYAN}Falling back to installing from source...${RESET}"
    echo ""
    install_from_source
}

# ─── Already installed flow ───────────────────────────────────────────────────
INSTALLED_VERSION=$(get_installed_version)
LATEST_VERSION=$(get_latest_version)

if [ -n "$INSTALLED_VERSION" ] && [ "$FORCE" = false ]; then
    echo -e "${GREEN}ldx $INSTALLED_VERSION is already installed.${RESET}"

    if [ -n "$LATEST_VERSION" ] && [ "$INSTALLED_VERSION" != "$LATEST_VERSION" ]; then
        echo -e "${YELLOW}Update available: $INSTALLED_VERSION → $LATEST_VERSION${RESET}"
        echo ""
        echo -e "  ${CYAN}1)${RESET} Update to $LATEST_VERSION"
        echo -e "  ${CYAN}2)${RESET} Reinstall current version"
        echo -e "  ${CYAN}3)${RESET} Exit"
        echo ""
        read -rp "Choice [1-3] (default: 1): " UPDATE_CHOICE
        UPDATE_CHOICE=${UPDATE_CHOICE:-1}

        case "$UPDATE_CHOICE" in
            1|2) ;; # continue to install
            3) echo -e "${CYAN}Exiting.${RESET}"; exit 0 ;;
            *) echo -e "${RED}Invalid choice. Exiting.${RESET}"; exit 1 ;;
        esac
    else
        if [ -n "$LATEST_VERSION" ]; then
            echo -e "${GREEN}You are up to date! ($LATEST_VERSION)${RESET}"
        else
            echo -e "${YELLOW}Could not check for updates (no network or no releases yet).${RESET}"
        fi
        echo ""
        echo -e "  ${CYAN}1)${RESET} Reinstall"
        echo -e "  ${CYAN}2)${RESET} Exit"
        echo ""
        read -rp "Choice [1-2] (default: 2): " REINSTALL_CHOICE
        REINSTALL_CHOICE=${REINSTALL_CHOICE:-2}

        case "$REINSTALL_CHOICE" in
            1) ;; # continue to install
            2) echo -e "${CYAN}Exiting.${RESET}"; exit 0 ;;
            *) echo -e "${RED}Invalid choice. Exiting.${RESET}"; exit 1 ;;
        esac
    fi
    echo ""
fi

# ─── Fresh install / update ───────────────────────────────────────────────────
if [ "$FROM_BINARY" = true ]; then
    install_from_binary
else
    install_from_source
fi

# ─── Done ─────────────────────────────────────────────────────────────────────
echo ""
echo -e "${GREEN}${BOLD}✓ ldx installed successfully!${RESET}"
echo -e "${CYAN}Try it: ${BOLD}ldx --help${RESET}"
echo ""
