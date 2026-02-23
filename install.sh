#!/usr/bin/env sh
# install.sh — Install or uninstall ldx (localdex)
# Usage: curl -sSf https://raw.githubusercontent.com/dylanisaiahp/localdex/main/install.sh | sh

# ─── Colors ───────────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
RESET='\033[0m'

REPO_URL="https://github.com/dylanisaiahp/localdex"
REPO_API="https://api.github.com/repos/dylanisaiahp/localdex/tags"
RELEASES_API="https://api.github.com/repos/dylanisaiahp/localdex/releases/latest"
BINARY_NAME="localdex"
ALIAS_NAME="ldx"
INSTALL_DIR="$HOME/.local/bin"

# ─── Helpers ──────────────────────────────────────────────────────────────────
print_header() {
    echo ""
    printf "${CYAN}${BOLD}🔍 ldx — Installer${RESET}\n"
    echo ""
}

get_installed_version() {
    if command -v ldx > /dev/null 2>&1; then
        ldx --version 2>/dev/null | grep -oE 'v[0-9]+\.[0-9]+\.[0-9]+' | head -1
    else
        echo ""
    fi
}

get_latest_version() {
    if command -v curl > /dev/null 2>&1; then
        curl -s "$REPO_API" 2>/dev/null \
            | grep '"name"' \
            | grep -oE 'v[0-9]+\.[0-9]+\.[0-9]+' \
            | sort -t. -k1,1V -k2,2n -k3,3n \
            | tail -1
    else
        echo ""
    fi
}

detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux)
            case "$ARCH" in
                x86_64) PLATFORM="ldx-linux-x86_64" ;;
                *) PLATFORM="" ;;
            esac
            ;;
        Darwin)
            case "$ARCH" in
                x86_64|arm64) PLATFORM="ldx-macos-x86_64" ;;
                *) PLATFORM="" ;;
            esac
            ;;
        MINGW*|MSYS*|CYGWIN*)
            PLATFORM="ldx-windows-x86_64.exe"
            ;;
        *) PLATFORM="" ;;
    esac
}

check_cargo() {
    if ! command -v cargo > /dev/null 2>&1; then
        printf "${YELLOW}Rust/cargo not found. Installing via rustup...${RESET}\n"
        echo ""
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        . "$HOME/.cargo/env"
        echo ""
        printf "${GREEN}✓ Rust installed${RESET}\n"
        echo ""
    fi
}

check_git() {
    if ! command -v git > /dev/null 2>&1; then
        printf "${RED}git not found. Please install git and try again.${RESET}\n"
        exit 1
    fi
}

install_config() {
    if [ -f "$INSTALL_DIR/config.toml" ]; then
        printf "${GREEN}✓ config.toml already present — skipping (aliases preserved)${RESET}\n"
    elif [ -f "default_config.toml" ]; then
        cp "default_config.toml" "$INSTALL_DIR/config.toml"
        printf "${GREEN}✓ config.toml generated${RESET}\n"
    else
        curl -sSf "https://raw.githubusercontent.com/dylanisaiahp/localdex/main/default_config.toml" \
            -o "$INSTALL_DIR/config.toml" 2>/dev/null \
            && printf "${GREEN}✓ config.toml downloaded${RESET}\n" \
            || printf "${YELLOW}⚠ Run ldx --sync after install to generate config${RESET}\n"
    fi
}

# ─── PATH setup ───────────────────────────────────────────────────────────────
setup_path() {
    # Already in PATH — nothing to do
    case ":$PATH:" in
        *":$INSTALL_DIR:"*) return 0 ;;
    esac

    # Detect shell rc file
    SHELL_NAME="$(basename "$SHELL")"
    case "$SHELL_NAME" in
        zsh)  RC_FILE="$HOME/.zshrc" ;;
        bash) RC_FILE="$HOME/.bashrc" ;;
        fish) RC_FILE="$HOME/.config/fish/config.fish" ; FISH=true ;;
        *)    RC_FILE="$HOME/.profile" ;;
    esac

    # Don't add if already present in rc file
    if grep -q "$INSTALL_DIR" "$RC_FILE" 2>/dev/null; then
        return 0
    fi

    echo "" >> "$RC_FILE"
    if [ "$FISH" = "true" ]; then
        echo "fish_add_path $INSTALL_DIR" >> "$RC_FILE"
    else
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$RC_FILE"
    fi
    printf "${GREEN}✓ Added $INSTALL_DIR to PATH in $RC_FILE${RESET}\n"
    printf "${YELLOW}  Restart your shell or run: source $RC_FILE${RESET}\n"
}

# ─── Binary download (fast path) ──────────────────────────────────────────────
try_download_binary() {
    detect_platform

    if [ -z "$PLATFORM" ]; then
        printf "${YELLOW}No pre-built binary for this platform — building from source${RESET}\n"
        return 1
    fi

    printf "${CYAN}Detected platform: $PLATFORM${RESET}\n"
    printf "${CYAN}Looking for pre-built binary...${RESET}\n"

    DOWNLOAD_URL=$(curl -s "$RELEASES_API" 2>/dev/null \
        | grep "browser_download_url" \
        | grep "$PLATFORM" \
        | grep -oE 'https://[^"]+' \
        | head -1)

    if [ -z "$DOWNLOAD_URL" ]; then
        printf "${YELLOW}No pre-built binary found — building from source${RESET}\n"
        return 1
    fi

    printf "${CYAN}Downloading binary...${RESET}\n"
    mkdir -p "$INSTALL_DIR"
    curl -sSfL "$DOWNLOAD_URL" -o "$INSTALL_DIR/$BINARY_NAME" 2>/dev/null

    if [ $? -ne 0 ]; then
        printf "${YELLOW}Download failed — building from source${RESET}\n"
        return 1
    fi

    chmod +x "$INSTALL_DIR/$BINARY_NAME"
    cp "$INSTALL_DIR/$BINARY_NAME" "$INSTALL_DIR/$ALIAS_NAME"
    printf "${GREEN}✓ Binary installed:  $INSTALL_DIR/$BINARY_NAME${RESET}\n"
    printf "${GREEN}✓ Alias installed:   $INSTALL_DIR/$ALIAS_NAME${RESET}\n"

    install_config
    setup_path
    return 0
}

# ─── Source location picker ───────────────────────────────────────────────────
pick_source_location() {
    echo ""
    printf "${BOLD}Where would you like to keep the source?${RESET}\n"
    echo ""
    printf "  ${CYAN}1)${RESET} $HOME/localdex-src\n"
    printf "  ${CYAN}2)${RESET} $HOME/Downloads/localdex-src\n"
    printf "  ${CYAN}3)${RESET} Custom path\n"
    echo ""
    printf "Choice [1-3] (default: 1): "
    read -r SRC_CHOICE </dev/tty
    SRC_CHOICE=${SRC_CHOICE:-1}

    case "$SRC_CHOICE" in
        1) SOURCE_PATH="$HOME/localdex-src" ;;
        2) SOURCE_PATH="$HOME/Downloads/localdex-src" ;;
        3)
            printf "Enter path: "
            read -r SOURCE_PATH </dev/tty
            ;;
        *) printf "${RED}Invalid choice.${RESET}\n"; exit 1 ;;
    esac
}

# ─── Build from source ────────────────────────────────────────────────────────
do_build_from_source() {
    KEEP_SOURCE="$1"

    check_cargo
    check_git

    if [ "$KEEP_SOURCE" = "true" ]; then
        pick_source_location
        CLONE_DIR="$SOURCE_PATH"
    else
        CLONE_DIR="$(mktemp -d)/localdex-src"
    fi

    echo ""
    printf "${CYAN}Cloning repository...${RESET}\n"
    git clone "$REPO_URL" "$CLONE_DIR"
    cd "$CLONE_DIR" || exit 1

    echo ""
    printf "${CYAN}Building from source...${RESET}\n"
    echo ""
    cargo build --release
    echo ""
    printf "${GREEN}✓ Build successful${RESET}\n"

    mkdir -p "$INSTALL_DIR"
    cp "target/release/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    cp "target/release/$BINARY_NAME" "$INSTALL_DIR/$ALIAS_NAME"
    printf "${GREEN}✓ Binary installed:  $INSTALL_DIR/$BINARY_NAME${RESET}\n"
    printf "${GREEN}✓ Alias installed:   $INSTALL_DIR/$ALIAS_NAME${RESET}\n"

    install_config
    setup_path

    cd "$HOME" || exit 1

    if [ "$KEEP_SOURCE" = "true" ]; then
        printf "${CYAN}✓ Source kept at: $CLONE_DIR${RESET}\n"
    else
        rm -rf "$CLONE_DIR"
        printf "${GREEN}✓ Source cleaned up${RESET}\n"
    fi
}

# ─── Install ──────────────────────────────────────────────────────────────────
do_install() {
    KEEP_SOURCE="$1"

    # Fast path: try pre-built binary first (skip if keeping source)
    if [ "$KEEP_SOURCE" = "false" ] && try_download_binary; then
        return 0
    fi

    # Slow path: build from source
    do_build_from_source "$KEEP_SOURCE"
}

# ─── Uninstall ────────────────────────────────────────────────────────────────
do_uninstall() {
    REMOVE_SOURCE="$1"

    if [ ! -f "$INSTALL_DIR/$BINARY_NAME" ] && ! command -v ldx > /dev/null 2>&1; then
        printf "${YELLOW}ldx not found — may already be uninstalled.${RESET}\n"
        exit 0
    fi

    INSTALLED_VERSION=$(get_installed_version)
    if [ -n "$INSTALLED_VERSION" ]; then
        printf "Found ldx ${BOLD}$INSTALLED_VERSION${RESET} at: ${CYAN}$INSTALL_DIR${RESET}\n"
    else
        printf "Found ldx at: ${CYAN}$INSTALL_DIR${RESET}\n"
    fi

    rm -f "$INSTALL_DIR/$BINARY_NAME" "$INSTALL_DIR/$ALIAS_NAME" "$INSTALL_DIR/config.toml"
    printf "${GREEN}✓ Removed binaries and config${RESET}\n"

    if [ "$REMOVE_SOURCE" = "true" ]; then
        echo ""
        printf "${BOLD}Where is the source code located?${RESET}\n"
        echo ""
        printf "  ${CYAN}1)${RESET} $HOME/localdex-src\n"
        printf "  ${CYAN}2)${RESET} $HOME/Downloads/localdex-src\n"
        printf "  ${CYAN}3)${RESET} Custom path\n"
        echo ""
        printf "Choice [1-3] (default: 1): "
        read -r SRC_CHOICE </dev/tty
        SRC_CHOICE=${SRC_CHOICE:-1}

        case "$SRC_CHOICE" in
            1) SOURCE_PATH="$HOME/localdex-src" ;;
            2) SOURCE_PATH="$HOME/Downloads/localdex-src" ;;
            3)
                printf "Enter path: "
                read -r SOURCE_PATH </dev/tty
                ;;
            *) printf "${RED}Invalid choice.${RESET}\n"; exit 1 ;;
        esac

        if [ -d "$SOURCE_PATH" ]; then
            rm -rf "$SOURCE_PATH"
            printf "${GREEN}✓ Source removed: $SOURCE_PATH${RESET}\n"
        else
            printf "${YELLOW}⚠ Source not found at $SOURCE_PATH — skipping${RESET}\n"
        fi
    fi
}

# ─── Main menu ────────────────────────────────────────────────────────────────
print_header

INSTALLED_VERSION=$(get_installed_version)
LATEST_VERSION=$(get_latest_version)

if [ -n "$INSTALLED_VERSION" ]; then
    printf "Installed: ${BOLD}$INSTALLED_VERSION${RESET}\n"
    if [ -n "$LATEST_VERSION" ] && [ "$INSTALLED_VERSION" != "$LATEST_VERSION" ]; then
        printf "${YELLOW}Update available: $INSTALLED_VERSION → $LATEST_VERSION${RESET}\n"
    else
        printf "${GREEN}Up to date${RESET}\n"
    fi
else
    printf "ldx is ${YELLOW}not installed${RESET}\n"
    [ -n "$LATEST_VERSION" ] && printf "Latest: ${BOLD}$LATEST_VERSION${RESET}\n"
fi

echo ""
printf "  ${CYAN}1)${RESET} Install\n"
printf "  ${CYAN}2)${RESET} Install + keep source\n"
printf "  ${CYAN}3)${RESET} Uninstall\n"
printf "  ${CYAN}4)${RESET} Uninstall + remove source\n"
printf "  ${CYAN}5)${RESET} Exit\n"
echo ""
printf "Choice [1-5]: "
read -r CHOICE </dev/tty

echo ""

case "$CHOICE" in
    1) do_install false ;;
    2) do_install true ;;
    3) do_uninstall false ;;
    4) do_uninstall true ;;
    5) printf "${CYAN}Exiting.${RESET}\n"; exit 0 ;;
    *) printf "${RED}Invalid choice.${RESET}\n"; exit 1 ;;
esac

echo ""

case "$CHOICE" in
    1|2)
        printf "${GREEN}${BOLD}✓ ldx installed successfully!${RESET}\n"
        echo ""
        printf "  ${CYAN}ldx --version${RESET}    # confirm install\n"
        printf "  ${CYAN}ldx --help${RESET}       # see available flags\n"
        printf "  ${CYAN}ldx --sync${RESET}       # ensure config is up to date\n"
        ;;
    3|4)
        printf "${GREEN}${BOLD}✓ ldx uninstalled successfully!${RESET}\n"
        ;;
esac

echo ""
