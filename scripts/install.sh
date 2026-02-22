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
REPO_API="https://api.github.com/repos/dylanisaiahp/localdex/tags"

# ─── Detect OS ────────────────────────────────────────────────────────────────
detect_os() {
    case "$(uname -s)" in
        Linux*)            echo "linux" ;;
        Darwin*)           echo "macos" ;;
        MINGW*|MSYS*|CYGWIN*) echo "windows" ;;
        *)                 echo "unknown" ;;
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
UNINSTALL=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --force)       FORCE=true;       shift ;;
        --from-source) FROM_SOURCE=true; shift ;;
        --binary)      FROM_BINARY=true; shift ;;
        --uninstall)   UNINSTALL=true;   shift ;;
        --help)
            echo "Usage: ./install.sh [options]"
            echo ""
            echo "Options:"
            echo "  --force          Force reinstall even if up to date"
            echo "  --from-source    Build from source (default)"
            echo "  --binary         Download pre-built binary (when available)"
            echo "  --uninstall      Uninstall ldx"
            echo "  --help           Show this help message"
            exit 0 ;;
        *)
            echo -e "${RED}Unknown argument: $1${RESET}"
            echo "Run ./install.sh --help for usage"
            exit 1 ;;
    esac
done

# ===========================================================================
# UNINSTALL
# ===========================================================================

do_uninstall() {
    echo ""
    echo -e "${CYAN}${BOLD}🔍 ldx — Uninstaller${RESET}"
    echo -e "${CYAN}OS: $OS${RESET}"
    echo ""

    if [ "$OS" = "windows" ]; then
        LOCATIONS=("$USERPROFILE/.cargo/bin" "$USERPROFILE/bin" "/c/Program Files/ldx")
    else
        LOCATIONS=("$HOME/.cargo/bin" "$HOME/.local/bin" "/usr/local/bin")
    fi

    FOUND_LOCATION=""
    for LOC in "${LOCATIONS[@]}"; do
        if [ -f "$LOC/$BINARY_NAME" ] || [ -f "$LOC/$ALIAS_NAME" ]; then
            FOUND_LOCATION="$LOC"
            break
        fi
    done

    if [ -z "$FOUND_LOCATION" ] && command -v ldx &> /dev/null; then
        FOUND_LOCATION=$(dirname "$(command -v ldx)")
    fi

    if [ -z "$FOUND_LOCATION" ]; then
        echo -e "${YELLOW}ldx not found — may already be uninstalled.${RESET}"
        echo ""
        exit 0
    fi

    if command -v ldx &> /dev/null; then
        VERSION=$(ldx --version 2>/dev/null | grep -oP 'v[\d.]+' || echo "")
        [ -n "$VERSION" ] \
            && echo -e "Found ldx ${BOLD}$VERSION${RESET} at: ${CYAN}$FOUND_LOCATION${RESET}" \
            || echo -e "Found ldx at: ${CYAN}$FOUND_LOCATION${RESET}"
    else
        echo -e "Found ldx at: ${CYAN}$FOUND_LOCATION${RESET}"
    fi

    CONFIG_PATH="$FOUND_LOCATION/config.toml"
    SOURCE_PATH=""
    [ -f "$CONFIG_PATH" ] && SOURCE_PATH=$(grep "source_path" "$CONFIG_PATH" 2>/dev/null | cut -d'"' -f2)
    HAS_SOURCE=false
    [ -n "$SOURCE_PATH" ] && [ -d "$SOURCE_PATH" ] && HAS_SOURCE=true

    echo ""
    echo -e "${BOLD}What would you like to do?${RESET}"
    echo ""

    if [ "$HAS_SOURCE" = true ]; then
        echo -e "  ${CYAN}1)${RESET} Uninstall binaries only (keep source)"
        echo -e "  ${CYAN}2)${RESET} Uninstall everything (binaries, config, and source)"
        echo -e "  ${CYAN}3)${RESET} Exit"
        echo ""
        if [ ! -t 0 ]; then CHOICE=; else read -rp "Choice [1-3] (default: 1): " CHOICE; fi
        CHOICE=${CHOICE:-1}
        case "$CHOICE" in
            1) REMOVE_SOURCE=false ;;
            2) REMOVE_SOURCE=true ;;
            3) echo -e "${CYAN}Exiting.${RESET}"; exit 0 ;;
            *) echo -e "${RED}Invalid choice.${RESET}"; exit 1 ;;
        esac
    else
        echo -e "  ${CYAN}1)${RESET} Uninstall ldx"
        echo -e "  ${CYAN}2)${RESET} Exit"
        echo ""
        if [ ! -t 0 ]; then CHOICE=; else read -rp "Choice [1-2] (default: 1): " CHOICE; fi
        CHOICE=${CHOICE:-1}
        REMOVE_SOURCE=false
        case "$CHOICE" in
            1) ;;
            2) echo -e "${CYAN}Exiting.${RESET}"; exit 0 ;;
            *) echo -e "${RED}Invalid choice.${RESET}"; exit 1 ;;
        esac
    fi

    echo ""
    echo -e "${YELLOW}Uninstalling ldx...${RESET}"
    echo ""

    NEEDS_SUDO=false
    [ "$FOUND_LOCATION" = "/usr/local/bin" ] && NEEDS_SUDO=true

    if [ "$NEEDS_SUDO" = true ]; then
        sudo rm -f "$FOUND_LOCATION/$BINARY_NAME" "$FOUND_LOCATION/$ALIAS_NAME"
    else
        rm -f "$FOUND_LOCATION/$BINARY_NAME" "$FOUND_LOCATION/$ALIAS_NAME"
    fi
    echo -e "${GREEN}✓ Removed binaries from $FOUND_LOCATION${RESET}"

    if [ -f "$CONFIG_PATH" ]; then
        [ "$NEEDS_SUDO" = true ] && sudo rm -f "$CONFIG_PATH" || rm -f "$CONFIG_PATH"
        echo -e "${GREEN}✓ Removed config.toml${RESET}"
    fi

    if [ "$REMOVE_SOURCE" = true ] && [ -d "$SOURCE_PATH" ]; then
        rm -rf "$SOURCE_PATH"
        echo -e "${GREEN}✓ Removed source directory${RESET}"
    fi

    echo ""
    echo -e "${YELLOW}Remove PATH entries added by install.sh? [y/N]:${RESET} "
    read -rp "" REMOVE_PATH
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
            sed -i.bak '/# ldx/d' "$SHELL_CONFIG"
            sed -i.bak "s|export PATH=\"\$PATH:$FOUND_LOCATION\"||" "$SHELL_CONFIG"
            rm -f "${SHELL_CONFIG}.bak"
            echo -e "${GREEN}✓ Removed PATH entries from $SHELL_CONFIG${RESET}"
        fi
    fi

    echo ""
    echo -e "${GREEN}${BOLD}✓ ldx uninstalled successfully!${RESET}"
    echo ""
}

if [ "$UNINSTALL" = true ]; then
    do_uninstall
    exit 0
fi

# ─── Header ───────────────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}${BOLD}🔍 ldx — Installer${RESET}"
echo -e "${CYAN}OS: $OS${RESET}"
echo ""

# ─── Version helpers ──────────────────────────────────────────────────────────
get_latest_version() {
    if command -v curl &> /dev/null; then
        curl -s "$REPO_API" 2>/dev/null | grep '"name"' | head -1 | grep -oP 'v[\d.]+' | head -1
    elif command -v wget &> /dev/null; then
        wget -qO- "$REPO_API" 2>/dev/null | grep '"name"' | head -1 | grep -oP 'v[\d.]+' | head -1
    else
        echo ""
    fi
}

get_installed_version() {
    if command -v ldx &> /dev/null; then
        ldx --version 2>/dev/null | grep -oP 'v[\d.]+' | head -1
    else
        echo ""
    fi
}

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
    if [ ! -t 0 ]; then CHOICE=; else read -rp "Choice [1-4] (default: 1): " CHOICE; fi
    CHOICE=${CHOICE:-1}

    case "$CHOICE" in
        1) DEST="$CARGO_BIN"; SKIP_PATH=true ;;
        2) DEST="$USER_BIN";  SKIP_PATH=false ;;
        3) DEST="$SYSTEM_BIN"; SKIP_PATH=false ;;
        4) read -rp "Enter custom path: " DEST; SKIP_PATH=false ;;
        *) echo -e "${RED}Invalid choice. Exiting.${RESET}"; exit 1 ;;
    esac
}

# ─── PATH setup ───────────────────────────────────────────────────────────────
setup_path() {
    if [ "${SKIP_PATH:-false}" = true ]; then
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

# ─── Generate config.toml from default_config.toml ───────────────────────────
generate_config() {
    local dest_dir="$1"
    local config_file="$dest_dir/config.toml"

    # Find default_config.toml — either in repo root or next to the binary
    local default_toml=""
    if [ -f "default_config.toml" ]; then
        default_toml="default_config.toml"
    elif [ -f "$dest_dir/default_config.toml" ]; then
        default_toml="$dest_dir/default_config.toml"
    fi

    if [ -f "$config_file" ]; then
        echo -e "${GREEN}✓ config.toml already present — skipping (aliases preserved)${RESET}"
    elif [ -n "$default_toml" ]; then
        cp "$default_toml" "$config_file"
        echo -e "${GREEN}✓ Generated config.toml from default_config.toml${RESET}"
    else
        echo -e "${YELLOW}⚠ default_config.toml not found — run ldx --sync after install${RESET}"
    fi
}

# ─── Install from source ──────────────────────────────────────────────────────
install_from_source() {
    if ! command -v cargo &> /dev/null; then
        echo -e "${YELLOW}Rust/cargo not found.${RESET}"
        echo -e "${CYAN}Installing Rust via rustup...${RESET}"
        echo ""
        if command -v curl &> /dev/null; then
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        elif command -v wget &> /dev/null; then
            wget -qO- https://sh.rustup.rs | sh -s -- -y
        else
            echo -e "${RED}Neither curl nor wget found. Please install Rust manually: https://rustup.rs${RESET}"
            exit 1
        fi
        if [ -f "$HOME/.cargo/env" ]; then
            source "$HOME/.cargo/env"
        fi
        echo ""
        echo -e "${GREEN}✓ Rust installed successfully!${RESET}"
        echo ""
    fi

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

    echo -e "${GREEN}✓ Binary installed: $DEST/$BINARY_NAME${RESET}"
    echo -e "${GREEN}✓ Alias installed:  $DEST/$ALIAS_NAME${RESET}"

    generate_config "$DEST"

    setup_path

    # Handle cloned source
    if [ -d "../localdex-src" ] || [ "$(basename "$PWD")" = "localdex-src" ]; then
        cd ..
        SOURCE_PATH="$(pwd)/localdex-src"

        echo ""
        echo -e "${YELLOW}Keep source code for future updates/modifications?${RESET}"
        if [ ! -t 0 ]; then KEEP_SRC=N; else read -rp "Keep source? [Y/n]: " KEEP_SRC; fi
        KEEP_SRC=${KEEP_SRC:-Y}

        if [[ "$KEEP_SRC" =~ ^[Yy]$ ]]; then
            echo -e "${CYAN}✓ Source kept at: $SOURCE_PATH${RESET}"
            CONFIG_FILE="$DEST/config.toml"
            if [ -f "$CONFIG_FILE" ] && ! grep -q "\[meta\]" "$CONFIG_FILE"; then
                echo "" >> "$CONFIG_FILE"
                echo "[meta]" >> "$CONFIG_FILE"
                echo "source_path = \"$SOURCE_PATH\"" >> "$CONFIG_FILE"
            fi
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
        if [ ! -t 0 ]; then UPDATE_CHOICE=; else read -rp "Choice [1-3] (default: 1): " UPDATE_CHOICE; fi
        UPDATE_CHOICE=${UPDATE_CHOICE:-1}
        case "$UPDATE_CHOICE" in
            1|2) ;;
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
        if [ ! -t 0 ]; then REINSTALL_CHOICE=; else read -rp "Choice [1-2] (default: 2): " REINSTALL_CHOICE; fi
        REINSTALL_CHOICE=${REINSTALL_CHOICE:-2}
        case "$REINSTALL_CHOICE" in
            1) ;;
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
echo ""
echo -e "${CYAN}${BOLD}Try it:${RESET}"
echo -e "  ${CYAN}ldx --version${RESET}          # confirm install"
echo -e "  ${CYAN}ldx --help${RESET}             # see available flags"
echo -e "  ${CYAN}ldx --check${RESET}            # validate your config"
echo -e "  ${CYAN}ldx --sync${RESET}             # ensure all default flags are present"
echo -e "  ${CYAN}ldx --edit${RESET}             # customize aliases or add your own flags"
echo ""
echo -e "Config: ${CYAN}$(dirname "$(command -v ldx)" 2>/dev/null || echo "\$DEST")/config.toml${RESET}  (or run ${CYAN}ldx --config${RESET})"
echo -e "Docs:   ${CYAN}https://github.com/dylanisaiahp/localdex${RESET}"
echo ""
