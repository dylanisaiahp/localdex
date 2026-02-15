#!/usr/bin/env bash
# build.sh — Build and install ldx
# Supports: Linux, macOS, Windows (Git Bash)
# Usage: ./build.sh [--debug] [--dest <path>]

set -e

# ─── Colors ───────────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
RESET='\033[0m'

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

# ─── Binary name ──────────────────────────────────────────────────────────────
if [ "$OS" = "windows" ]; then
    BINARY_NAME="localdex.exe"
    ALIAS_NAME="ldx.exe"
else
    BINARY_NAME="localdex"
    ALIAS_NAME="ldx"
fi

# ─── Parse arguments ──────────────────────────────────────────────────────────
BUILD_MODE="release"
CUSTOM_DEST=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --debug)
            BUILD_MODE="debug"
            shift ;;
        --release)
            BUILD_MODE="release"
            shift ;;
        --dest)
            CUSTOM_DEST="$2"
            shift 2 ;;
        *)
            echo -e "${RED}Unknown argument: $1${RESET}"
            echo "Usage: ./build.sh [--debug] [--release] [--dest <path>]"
            exit 1 ;;
    esac
done

# ─── Header ───────────────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}${BOLD}🔍 ldx — Build Script${RESET}"
echo -e "${CYAN}OS: $OS | Mode: $BUILD_MODE${RESET}"
echo ""

# ─── Check for Rust/cargo ─────────────────────────────────────────────────────
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}cargo not found. Please install Rust: https://rustup.rs${RESET}"
    exit 1
fi

# ─── Destination options ──────────────────────────────────────────────────────
if [ -n "$CUSTOM_DEST" ]; then
    DEST="$CUSTOM_DEST"
    SKIP_PATH=false
else
    echo -e "${BOLD}Where would you like to install ldx?${RESET}"
    echo ""

    if [ "$OS" = "windows" ]; then
        CARGO_BIN="$USERPROFILE/.cargo/bin"
        USER_BIN="$USERPROFILE/bin"
        SYSTEM_BIN="/c/Program Files/ldx"
        echo -e "  ${CYAN}1)${RESET} ${CARGO_BIN} ${GREEN}(default, already in PATH)${RESET}"
        echo -e "  ${CYAN}2)${RESET} ${USER_BIN}"
        echo -e "  ${CYAN}3)${RESET} ${SYSTEM_BIN} ${YELLOW}(may require admin)${RESET}"
        echo -e "  ${CYAN}4)${RESET} Custom path"
    else
        CARGO_BIN="$HOME/.cargo/bin"
        USER_BIN="$HOME/.local/bin"
        SYSTEM_BIN="/usr/local/bin"
        echo -e "  ${CYAN}1)${RESET} ${CARGO_BIN} ${GREEN}(default, already in PATH)${RESET}"
        echo -e "  ${CYAN}2)${RESET} ${USER_BIN}"
        echo -e "  ${CYAN}3)${RESET} ${SYSTEM_BIN} ${YELLOW}(requires sudo)${RESET}"
        echo -e "  ${CYAN}4)${RESET} Custom path"
    fi

    echo ""
    read -rp "Choice [1-4] (default: 1): " CHOICE
    CHOICE=${CHOICE:-1}

    case "$CHOICE" in
        1)
            DEST="$CARGO_BIN"
            SKIP_PATH=true ;;
        2)
            DEST="$USER_BIN"
            SKIP_PATH=false ;;
        3)
            DEST="$SYSTEM_BIN"
            SKIP_PATH=false ;;
        4)
            read -rp "Enter custom path: " DEST
            SKIP_PATH=false ;;
        *)
            echo -e "${RED}Invalid choice. Exiting.${RESET}"
            exit 1 ;;
    esac
fi

# ─── Build ────────────────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}Building in ${BOLD}${BUILD_MODE}${RESET}${CYAN} mode...${RESET}"
echo ""

if [ "$BUILD_MODE" = "release" ]; then
    cargo build --release
    BUILD_DIR="target/release"
else
    cargo build
    BUILD_DIR="target/debug"
fi

echo ""
echo -e "${GREEN}✓ Build successful${RESET}"

# ─── Create destination directory ─────────────────────────────────────────────
mkdir -p "$DEST"

# ─── Copy binary ──────────────────────────────────────────────────────────────
echo -e "${CYAN}Installing to ${BOLD}${DEST}${RESET}${CYAN}...${RESET}"

if [ "$OS" = "windows" ] || [ "$CHOICE" != "3" ]; then
    cp "$BUILD_DIR/$BINARY_NAME" "$DEST/$BINARY_NAME"
    cp "$BUILD_DIR/$BINARY_NAME" "$DEST/$ALIAS_NAME"
else
    sudo cp "$BUILD_DIR/$BINARY_NAME" "$DEST/$BINARY_NAME"
    sudo cp "$BUILD_DIR/$BINARY_NAME" "$DEST/$ALIAS_NAME"
fi

echo -e "${GREEN}✓ Binary installed: ${DEST}/${BINARY_NAME}${RESET}"
echo -e "${GREEN}✓ Alias installed:  ${DEST}/${ALIAS_NAME}${RESET}"

# ─── Copy config.toml if it exists ────────────────────────────────────────────
if [ -f "config.toml" ]; then
    cp "config.toml" "$DEST/config.toml"
    echo -e "${GREEN}✓ config.toml copied${RESET}"
fi

# ─── PATH setup (skip for option 1) ───────────────────────────────────────────
if [ "$SKIP_PATH" = false ]; then
    # Check if already in PATH
    if echo "$PATH" | grep -q "$DEST"; then
        echo -e "${GREEN}✓ ${DEST} is already in PATH${RESET}"
    else
        echo ""
        echo -e "${YELLOW}${DEST} is not in your PATH.${RESET}"
        read -rp "Add it now? [y/N]: " ADD_PATH
        if [[ "$ADD_PATH" =~ ^[Yy]$ ]]; then
            SHELL_CONFIG=""
            if [ "$OS" = "windows" ]; then
                # On Git Bash, update .bashrc
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
            echo -e "${GREEN}✓ Added to ${SHELL_CONFIG}${RESET}"
            echo -e "${YELLOW}Run: source ${SHELL_CONFIG}${RESET}"
        fi
    fi
fi

# ─── Done ─────────────────────────────────────────────────────────────────────
echo ""
echo -e "${GREEN}${BOLD}✓ ldx installed successfully!${RESET}"
echo -e "${CYAN}Try it: ${BOLD}ldx --help${RESET}"
echo ""
