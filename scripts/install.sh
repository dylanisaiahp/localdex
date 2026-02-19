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
        curl -s "$REPO_API" 2>/dev/null | grep '"name"' | head -1 | grep -oP 'v[\d.]+' | head -1
    elif command -v wget &> /dev/null; then
        wget -qO- "$REPO_API" 2>/dev/null | grep '"name"' | head -1 | grep -oP 'v[\d.]+' | head -1
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
    # Check for cargo — auto-install rustup if missing
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
        # Source cargo env so it's available in this session
        if [ -f "$HOME/.cargo/env" ]; then
            source "$HOME/.cargo/env"
        fi
        echo ""
        echo -e "${GREEN}✓ Rust installed successfully!${RESET}"
        echo ""
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

    # Generate config.toml
    CONFIG_FILE="$DEST/config.toml"
    cat > "$CONFIG_FILE" << 'CONFIGEOF'
# ═══════════════════════════════════════════════════════════════════════════
# ldx configuration file (v0.0.5+)
# ═══════════════════════════════════════════════════════════════════════════
# Edit this file to customize flags, create aliases, and define custom commands
#
# os values: "all", "windows", "linux", "macos"
# action types: "set_boolean", "set_value"

# ═══════════════════════════════════════════════════════════════════════════
# Built-in flags (editable)
# ═══════════════════════════════════════════════════════════════════════════

[flags.all-files]
short = "a"
long = "all-files"
description = "Count all files, no filter needed"
os = "all"
action = "set_boolean"
target = "all"

[flags.all-drives]
short = "A"
long = "all-drives"
description = "Scan all drives with a per-drive breakdown and total"
os = "windows"
action = "set_boolean"
target = "all_drives"

[flags.case-sensitive]
short = "s"
long = "case-sensitive"
description = "Case-sensitive search"
os = "all"
action = "set_boolean"
target = "case_sensitive"

[flags.dir]
short = "d"
long = "dir"
description = "Directory to search in (default: current)"
os = "all"
action = "set_value"
target = "dir"

[flags.dirs]
short = "D"
long = "dirs"
description = "Search for directories instead of files"
os = "all"
action = "set_boolean"
target = "dirs_only"

[flags.extension]
short = "e"
long = "extension"
description = "Search by file extension (e.g. pdf, rs)"
os = "all"
action = "set_value"
target = "extension"

[flags.first]
short = "1"
long = "first"
description = "Stop after the first match"
os = "all"
action = "set_boolean"
target = "first"

[flags.help]
short = "h"
long = "help"
description = "Show this help message"
os = "all"
action = "show_help"

[flags.limit]
short = "L"
long = "limit"
description = "Stop after N matches (e.g. -L 5)"
os = "all"
action = "set_value"
target = "limit"

[flags.open]
short = "o"
long = "open"
description = "Open or launch the matched file"
os = "all"
action = "set_boolean"
target = "open"

[flags.quiet]
short = "q"
long = "quiet"
description = "Suppress per-file output; still prints summary count"
os = "all"
action = "set_boolean"
target = "quiet"

[flags.stats]
short = "S"
long = "stats"
description = "Show scan statistics"
os = "all"
action = "set_boolean"
target = "stats"

[flags.threads]
short = "t"
long = "threads"
description = "Number of threads to use (default: all available)"
os = "all"
action = "set_value"
target = "threads"

[flags.verbose]
short = "v"
long = "verbose"
description = "Show detailed scan breakdown (files + dirs separately)"
os = "all"
action = "set_boolean"
target = "verbose"

[flags.where]
short = "w"
long = "where"
description = "Print the path with cd hint (implies -1)"
os = "all"
action = "set_boolean"
target = "where_mode"

# ═══════════════════════════════════════════════════════════════════════════
# Custom flags (user-defined shortcuts)
# ═══════════════════════════════════════════════════════════════════════════
# Uncomment examples below or add your own!

# [custom.pdf]
# short = "P"
# long = "pdf"
# description = "Search for PDF files"
# os = "all"
# action = "set_value"
# target = "extension"
# value = "pdf"

# [custom.music]
# short = "M"
# long = "music"
# description = "Search for music files"
# os = "all"
# action = "set_value"
# target = "extension"
# value = "mp3"

# ═══════════════════════════════════════════════════════════════════════════
# Aliases (expand to multiple flags)
# ═══════════════════════════════════════════════════════════════════════════
# Uncomment examples below or add your own!

# [aliases]
# docs = "-e pdf -e docx -e txt"
# fast = "-t 16 -q"
# home = "-d ~ -S"
CONFIGEOF

    echo -e "${GREEN}✓ Generated config.toml${RESET}"

    setup_path

    # Clean up cloned dir if we cloned
    CLONED_SOURCE=false
    SOURCE_PATH=""
    if [ -d "../localdex-src" ] || [ "$(basename "$PWD")" = "localdex-src" ]; then
        CLONED_SOURCE=true
        cd ..
        SOURCE_PATH="$(pwd)/localdex-src"
        
        echo ""
        echo -e "${YELLOW}Keep source code for future updates/modifications?${RESET}"
        read -rp "Keep source? [Y/n]: " KEEP_SRC
        KEEP_SRC=${KEEP_SRC:-Y}
        
        if [[ "$KEEP_SRC" =~ ^[Yy]$ ]]; then
            echo -e "${CYAN}✓ Source kept at: $SOURCE_PATH${RESET}"
            
            # Save source path to config.toml
            CONFIG_FILE="$DEST/config.toml"
            if [ -f "$CONFIG_FILE" ]; then
                # Add meta section if it doesn't exist
                if ! grep -q "\[meta\]" "$CONFIG_FILE"; then
                    echo "" >> "$CONFIG_FILE"
                    echo "[meta]" >> "$CONFIG_FILE"
                    echo "source_path = "$SOURCE_PATH"" >> "$CONFIG_FILE"
                fi
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
echo ""
echo -e "${CYAN}${BOLD}localdex v0.0.7 installed! 🚀${RESET}"
echo ""
echo -e "${BOLD}Quick next steps:${RESET}"
echo -e "  ${CYAN}ldx --version${RESET}          # confirm install"
echo -e "  ${CYAN}ldx --help${RESET}             # see the new dynamic help"
echo -e "  ${CYAN}ldx --check${RESET}            # validate your config.toml"
echo -e "  ${CYAN}ldx --sync${RESET}             # ensure all default flags are present"
echo -e "  ${CYAN}ldx --edit${RESET}             # customize aliases or add your own flags"
echo ""
echo -e "Config location: ${CYAN}$(dirname "$(command -v ldx)")/config.toml${RESET}  (or run ${CYAN}ldx --config${RESET})"
echo -e "Docs & source:   ${CYAN}https://github.com/dylanisaiahp/localdex${RESET}"
echo ""
echo -e "${YELLOW}Note: Pre-built binaries coming in future releases — for now, enjoy the source-built speed!${RESET}"
echo ""
