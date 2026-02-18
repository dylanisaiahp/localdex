#!/usr/bin/env bash
# uninstall.sh — Uninstall ldx (localdex)
# Supports: Linux, macOS, Windows (Git Bash)
# Usage: ./uninstall.sh

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

# ─── Binary names ─────────────────────────────────────────────────────────────
if [ "$OS" = "windows" ]; then
    BINARY_NAME="localdex.exe"
    ALIAS_NAME="ldx.exe"
else
    BINARY_NAME="localdex"
    ALIAS_NAME="ldx"
fi

# ─── Header ───────────────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}${BOLD}🔍 ldx — Uninstaller${RESET}"
echo -e "${CYAN}OS: $OS${RESET}"
echo ""

# ─── Check standard locations ─────────────────────────────────────────────────
if [ "$OS" = "windows" ]; then
    LOCATIONS=(
        "$USERPROFILE/.cargo/bin"
        "$USERPROFILE/bin"
        "/c/Program Files/ldx"
    )
else
    LOCATIONS=(
        "$HOME/.cargo/bin"
        "$HOME/.local/bin"
        "/usr/local/bin"
    )
fi

FOUND_LOCATION=""
for LOC in "${LOCATIONS[@]}"; do
    if [ -f "$LOC/$BINARY_NAME" ] || [ -f "$LOC/$ALIAS_NAME" ]; then
        FOUND_LOCATION="$LOC"
        break
    fi
done

# ─── If not found in standard locations, use ldx to find itself ───────────────
if [ -z "$FOUND_LOCATION" ]; then
    echo -e "${YELLOW}ldx not found in standard locations.${RESET}"
    
    if command -v ldx &> /dev/null; then
        echo -e "${CYAN}Searching for ldx using ldx...${RESET}"
        
        # Find ldx binary
        LDX_PATH=$(command -v ldx)
        
        if [ -n "$LDX_PATH" ]; then
            FOUND_LOCATION=$(dirname "$LDX_PATH")
            echo -e "${GREEN}Found: $LDX_PATH${RESET}"
        fi
    fi
fi

# ─── If still not found, exit ─────────────────────────────────────────────────
if [ -z "$FOUND_LOCATION" ]; then
    echo -e "${YELLOW}ldx not found in standard locations or PATH.${RESET}"
    echo -e "${CYAN}It may have already been uninstalled.${RESET}"
    echo ""
    exit 0
fi

# ─── Get version if possible ──────────────────────────────────────────────────
if command -v ldx &> /dev/null; then
    VERSION=$(ldx --version 2>/dev/null | grep -oP 'v[\d.]+' || echo "")
    if [ -n "$VERSION" ]; then
        echo -e "Found ldx ${BOLD}$VERSION${RESET} at: ${CYAN}$FOUND_LOCATION${RESET}"
    else
        echo -e "Found ldx at: ${CYAN}$FOUND_LOCATION${RESET}"
    fi
else
    echo -e "Found ldx at: ${CYAN}$FOUND_LOCATION${RESET}"
fi

# ─── Check for config.toml and source path ───────────────────────────────────
CONFIG_PATH="$FOUND_LOCATION/config.toml"
HAS_CONFIG=false
SOURCE_PATH=""

if [ -f "$CONFIG_PATH" ]; then
    HAS_CONFIG=true
    # Try to extract source path from config
    SOURCE_PATH=$(grep "source_path" "$CONFIG_PATH" 2>/dev/null | cut -d'"' -f2)
fi

HAS_SOURCE=false
if [ -n "$SOURCE_PATH" ] && [ -d "$SOURCE_PATH" ]; then
    HAS_SOURCE=true
fi

echo ""
echo -e "${BOLD}What would you like to do?${RESET}"
echo ""

if [ "$HAS_SOURCE" = true ]; then
    echo -e "  ${CYAN}1)${RESET} Uninstall binaries only (keep source)"
    echo -e "  ${CYAN}2)${RESET} Uninstall everything (binaries, config, and source)"
    echo -e "  ${CYAN}3)${RESET} Exit"
    echo ""
    read -rp "Choice [1-3] (default: 1): " CHOICE
    CHOICE=${CHOICE:-1}
else
    echo -e "  ${CYAN}1)${RESET} Uninstall ldx"
    echo -e "  ${CYAN}2)${RESET} Exit"
    echo ""
    read -rp "Choice [1-2] (default: 1): " CHOICE
    CHOICE=${CHOICE:-1}
fi

# Config is always removed (regenerates automatically)
REMOVE_CONFIG=true
REMOVE_SOURCE=false

if [ "$HAS_SOURCE" = true ]; then
    case "$CHOICE" in
        1) REMOVE_SOURCE=false ;;
        2) REMOVE_SOURCE=true ;;
        3) echo -e "${CYAN}Exiting.${RESET}"; exit 0 ;;
        *) echo -e "${RED}Invalid choice. Exiting.${RESET}"; exit 1 ;;
    esac
else
    case "$CHOICE" in
        1) ;; # Just uninstall
        2) echo -e "${CYAN}Exiting.${RESET}"; exit 0 ;;
        *) echo -e "${RED}Invalid choice. Exiting.${RESET}"; exit 1 ;;
    esac
fi

echo ""
echo -e "${YELLOW}Uninstalling ldx...${RESET}"
echo ""

# ─── Remove binaries ──────────────────────────────────────────────────────────
NEEDS_SUDO=false
if [ "$FOUND_LOCATION" = "/usr/local/bin" ]; then
    NEEDS_SUDO=true
fi

if [ "$NEEDS_SUDO" = true ]; then
    sudo rm -f "$FOUND_LOCATION/$BINARY_NAME"
    sudo rm -f "$FOUND_LOCATION/$ALIAS_NAME"
else
    rm -f "$FOUND_LOCATION/$BINARY_NAME"
    rm -f "$FOUND_LOCATION/$ALIAS_NAME"
fi

echo -e "${GREEN}✓ Removed $FOUND_LOCATION/$BINARY_NAME${RESET}"
echo -e "${GREEN}✓ Removed $FOUND_LOCATION/$ALIAS_NAME${RESET}"

# ─── Remove config if requested ───────────────────────────────────────────────
if [ "$REMOVE_CONFIG" = true ] && [ -f "$CONFIG_PATH" ]; then
    if [ "$NEEDS_SUDO" = true ]; then
        sudo rm -f "$CONFIG_PATH"
    else
        rm -f "$CONFIG_PATH"
    fi
    echo -e "${GREEN}✓ Removed $CONFIG_PATH${RESET}"
fi

# ─── Remove source if requested ──────────────────────────────────────────────
if [ "$REMOVE_SOURCE" = true ] && [ -d "$SOURCE_PATH" ]; then
    echo -e "${YELLOW}Removing source at: $SOURCE_PATH${RESET}"
    rm -rf "$SOURCE_PATH"
    echo -e "${GREEN}✓ Removed source directory${RESET}"
fi

# ─── Clean up PATH entries (optional) ─────────────────────────────────────────
echo ""
echo -e "${YELLOW}Would you like to remove PATH entries for ldx?${RESET}"
echo -e "${CYAN}(Only affects entries added by install.sh)${RESET}"
read -rp "Remove PATH entries? [y/N]: " REMOVE_PATH

if [[ "$REMOVE_PATH" =~ ^[Yy]$ ]]; then
    if [ "$OS" = "windows" ]; then
        SHELL_CONFIG="$HOME/.bashrc"
    elif [ -n "$ZSH_VERSION" ] || [ "$SHELL" = "/bin/zsh" ] || [ "$SHELL" = "/usr/bin/zsh" ]; then
        SHELL_CONFIG="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        SHELL_CONFIG="$HOME/.bashrc"
    else
        SHELL_CONFIG="$HOME/.profile"
    fi
    
    if [ -f "$SHELL_CONFIG" ]; then
        # Remove ldx PATH entries
        sed -i.bak '/# ldx/d' "$SHELL_CONFIG"
        sed -i.bak "\|export PATH=\"\$PATH:$FOUND_LOCATION\"|d" "$SHELL_CONFIG"
        rm -f "${SHELL_CONFIG}.bak"
        echo -e "${GREEN}✓ Removed PATH entries from $SHELL_CONFIG${RESET}"
    fi
fi

# ─── Done ─────────────────────────────────────────────────────────────────────
echo ""
echo -e "${GREEN}${BOLD}✓ ldx uninstalled successfully!${RESET}"
echo ""
