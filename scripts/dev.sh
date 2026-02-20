#!/usr/bin/env bash
# dev.sh — Developer tool for ldx
# Combines: build, benchmark, bump
# Supports: Linux, macOS, Windows (Git Bash)
# Usage: ./scripts/dev.sh <command> [options]

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

# ─── Help ─────────────────────────────────────────────────────────────────────
show_help() {
    echo ""
    echo -e "${CYAN}${BOLD}🔍 ldx — Developer Tool${RESET}"
    echo ""
    echo -e "${BOLD}Usage:${RESET} ./scripts/dev.sh <command> [options]"
    echo ""
    echo -e "${BOLD}Commands:${RESET}"
    echo -e "  ${CYAN}build${RESET}        Build and install ldx locally"
    echo -e "  ${CYAN}benchmark${RESET}    Run performance benchmarks"
    echo -e "  ${CYAN}bump${RESET}         Suggest and apply a version bump"
    echo ""
    echo -e "${BOLD}Build options:${RESET}"
    echo -e "  ${CYAN}--debug${RESET}      Build in debug mode (default: release)"
    echo -e "  ${CYAN}--dest PATH${RESET}  Install to a custom path"
    echo ""
    echo -e "${BOLD}Benchmark options:${RESET}"
    echo -e "  ${CYAN}--runs N${RESET}     Runs per combination (default: 10)"
    echo -e "  ${CYAN}--cold${RESET}       Label as cold cache run"
    echo -e "  ${CYAN}--warm${RESET}       Label as warm cache run (default)"
    echo -e "  ${CYAN}--threads LIST${RESET} Comma-separated thread counts"
    echo -e "  ${CYAN}--dirs LIST${RESET}  Comma-separated directories"
    echo -e "  ${CYAN}--out FILE${RESET}   Custom output filename (without extension)"
    echo -e "  ${CYAN}--live${RESET}       Print live table as benchmark runs"
    echo -e "  ${CYAN}--csv${RESET}        Also save raw CSV alongside the markdown report"
    echo ""
    echo -e "${BOLD}Examples:${RESET}"
    echo -e "  ${CYAN}./scripts/dev.sh build${RESET}"
    echo -e "  ${CYAN}./scripts/dev.sh build --debug${RESET}"
    echo -e "  ${CYAN}./scripts/dev.sh benchmark --runs 20 --cold${RESET}"
    echo -e "  ${CYAN}./scripts/dev.sh benchmark --dirs /home,/usr${RESET}"
    echo -e "  ${CYAN}./scripts/dev.sh bump${RESET}"
    echo ""
}

# ─── Check we're in the repo root ─────────────────────────────────────────────
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Cargo.toml not found. Run this from the repo root.${RESET}"
    exit 1
fi

# ===========================================================================
# BUILD
# ===========================================================================

cmd_build() {
    BUILD_MODE="release"
    CUSTOM_DEST=""

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --debug)   BUILD_MODE="debug"; shift ;;
            --release) BUILD_MODE="release"; shift ;;
            --dest)    CUSTOM_DEST="$2"; shift 2 ;;
            *)
                echo -e "${RED}Unknown build argument: $1${RESET}"
                echo "Usage: ./scripts/dev.sh build [--debug] [--release] [--dest <path>]"
                exit 1 ;;
        esac
    done

    echo ""
    echo -e "${CYAN}${BOLD}🔍 ldx — Build${RESET}"
    echo -e "${CYAN}OS: $OS | Mode: $BUILD_MODE${RESET}"
    echo ""

    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}cargo not found. Please install Rust: https://rustup.rs${RESET}"
        exit 1
    fi

    # Destination
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
            1) DEST="$CARGO_BIN"; SKIP_PATH=true ;;
            2) DEST="$USER_BIN";  SKIP_PATH=false ;;
            3) DEST="$SYSTEM_BIN"; SKIP_PATH=false ;;
            4) read -rp "Enter custom path: " DEST; SKIP_PATH=false ;;
            *) echo -e "${RED}Invalid choice. Exiting.${RESET}"; exit 1 ;;
        esac
    fi

    # Build
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

    # Install
    mkdir -p "$DEST"
    echo -e "${CYAN}Installing to ${BOLD}${DEST}${RESET}${CYAN}...${RESET}"
    if [ "$OS" = "windows" ] || [ "${CHOICE:-1}" != "3" ]; then
        cp "$BUILD_DIR/$BINARY_NAME" "$DEST/$BINARY_NAME"
        cp "$BUILD_DIR/$BINARY_NAME" "$DEST/$ALIAS_NAME"
    else
        sudo cp "$BUILD_DIR/$BINARY_NAME" "$DEST/$BINARY_NAME"
        sudo cp "$BUILD_DIR/$BINARY_NAME" "$DEST/$ALIAS_NAME"
    fi
    echo -e "${GREEN}✓ Binary installed: ${DEST}/${BINARY_NAME}${RESET}"
    echo -e "${GREEN}✓ Alias installed:  ${DEST}/${ALIAS_NAME}${RESET}"

    # Generate config.toml from default_config.toml
    if [ -f "default_config.toml" ]; then
        cp "default_config.toml" "$DEST/config.toml"
        echo -e "${GREEN}✓ config.toml generated from default_config.toml${RESET}"
    elif [ -f "$DEST/config.toml" ]; then
        echo -e "${GREEN}✓ config.toml already present — skipping${RESET}"
    fi

    # PATH setup
    if [ "${SKIP_PATH:-false}" = false ]; then
        if echo "$PATH" | grep -q "$DEST"; then
            echo -e "${GREEN}✓ ${DEST} is already in PATH${RESET}"
        else
            echo ""
            echo -e "${YELLOW}${DEST} is not in your PATH.${RESET}"
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
                echo -e "${GREEN}✓ Added to ${SHELL_CONFIG}${RESET}"
                echo -e "${YELLOW}Run: source ${SHELL_CONFIG}${RESET}"
            fi
        fi
    fi

    echo ""
    echo -e "${GREEN}${BOLD}✓ ldx installed successfully!${RESET}"
    echo -e "${CYAN}Try it: ${BOLD}ldx --help${RESET}"
    echo ""
}

# ===========================================================================
# BENCHMARK
# ===========================================================================

cmd_benchmark() {
    RUNS=10
    CACHE_TYPE="warm"
    THREAD_LIST="1,2,4,6,8,10,12,14,16"
    CUSTOM_DIRS=""
    CUSTOM_OUT=""
    LIVE=false
    CSV=false

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --runs)    RUNS="$2"; shift 2 ;;
            --cold)    CACHE_TYPE="cold"; shift ;;
            --warm)    CACHE_TYPE="warm"; shift ;;
            --threads) THREAD_LIST="$2"; shift 2 ;;
            --dirs)    CUSTOM_DIRS="$2"; shift 2 ;;
            --out)     CUSTOM_OUT="$2"; shift 2 ;;
            --live)    LIVE=true; shift ;;
            --csv)     CSV=true; shift ;;
            *)
                echo -e "${RED}Unknown benchmark argument: $1${RESET}"
                echo "Run ./scripts/dev.sh --help for usage"
                exit 1 ;;
        esac
    done

    if ! command -v ldx &> /dev/null; then
        echo -e "${RED}ldx not found in PATH. Run: ./scripts/dev.sh build${RESET}"
        exit 1
    fi

    if [ -n "$CUSTOM_DIRS" ]; then
        IFS=',' read -ra DIRS <<< "$CUSTOM_DIRS"
    else
        if [ "$OS" = "windows" ]; then
            DIRS=("C:\\" "C:\\Users\\$USERNAME" "C:\\Program Files" "D:\\")
        elif [ "$OS" = "macos" ]; then
            DIRS=("$HOME" "/usr" "/Applications")
        else
            DIRS=("$HOME" "/usr" "/etc")
        fi
    fi

    IFS=',' read -ra THREADS <<< "$THREAD_LIST"

    get_cpu() {
        if [ "$OS" = "windows" ]; then
            powershell.exe -command "Get-WmiObject Win32_Processor | Select-Object -ExpandProperty Name" 2>/dev/null | tr -d '\r' | xargs
        elif [ "$OS" = "macos" ]; then
            sysctl -n machdep.cpu.brand_string 2>/dev/null | xargs
        else
            grep "model name" /proc/cpuinfo 2>/dev/null | head -1 | cut -d':' -f2 | xargs
        fi
    }

    CPU=$(get_cpu)
    CPU_SHORT=$(echo "$CPU" | grep -oP '(i\d-\d+\w*|Ryzen \d \d+\w*|Apple M\d+)' | head -1)
    [ -z "$CPU_SHORT" ] && CPU_SHORT="unknown-cpu"

    DATE=$(date +%Y-%m-%d)
    MD_FILE="${CUSTOM_OUT:-benchmark_${CACHE_TYPE}_${RUNS}runs_${CPU_SHORT}_${DATE}.md}"
    CSV_FILE="${MD_FILE%.md}.csv"

    TOTAL_COMBOS=$(( ${#DIRS[@]} * ${#THREADS[@]} ))
    TOTAL_RUNS=$(( TOTAL_COMBOS * RUNS ))

    echo ""
    echo -e "${CYAN}${BOLD}ldx Benchmark${RESET}"
    echo -e "${CYAN}OS: $OS | Cache: $CACHE_TYPE | Runs: $RUNS per combo | CPU: $CPU_SHORT${RESET}"
    echo ""

    [ "$CSV" = true ] && echo "Directory,Threads,Runs,Avg,Median,Min,Max,AllSpeeds" > "$CSV_FILE"

    # Collect all results for markdown summary
    declare -a MD_RESULTS

    calc_stats() {
        local speeds_str="$1"
        echo "$speeds_str" | tr ';' '\n' | awk '
        BEGIN { n=0; sum=0; min=999999999; max=0 }
        {
            val = $1 + 0
            arr[n++] = val
            sum += val
            if (val < min) min = val
            if (val > max) max = val
        }
        END {
            avg = int(sum / n)
            for (i = 0; i < n; i++)
                for (j = i+1; j < n; j++)
                    if (arr[i] > arr[j]) { tmp=arr[i]; arr[i]=arr[j]; arr[j]=tmp }
            if (n % 2 == 0)
                median = int((arr[n/2-1] + arr[n/2]) / 2)
            else
                median = arr[int(n/2)]
            print avg "," median "," min "," max
        }'
    }

    fmt_num() {
        echo "$1" | sed ':a;s/\B[0-9]\{3\}\>/,&/;ta'
    }

    CURRENT_DIR=""
    CURRENT_RUN=0

    for DIR in "${DIRS[@]}"; do
        for T in "${THREADS[@]}"; do
            SPEEDS=""

            # Print directory header on first thread for that dir
            if [ "$DIR" != "$CURRENT_DIR" ]; then
                CURRENT_DIR="$DIR"
                if [ "$LIVE" = true ]; then
                    echo ""
                    echo -e "${BOLD}  $DIR${RESET}"
                    printf "  %-10s %14s %14s %14s %14s\n" "Threads" "Avg" "Median" "Min" "Max"
                    printf "  %s\n" "----------------------------------------------------------------------"
                else
                    echo -ne "  Benchmarking ${CYAN}$DIR${RESET}..."
                fi
            fi

            for (( i=1; i<=RUNS; i++ )); do
                CURRENT_RUN=$(( CURRENT_RUN + 1 ))
                if [ "$LIVE" = false ]; then
                    printf "\r  Benchmarking ${CYAN}%-40s${RESET} [%d/%d]" "$DIR" "$CURRENT_RUN" "$TOTAL_RUNS"
                fi
                OUTPUT=$(ldx -a -q -S -d "$DIR" -t "$T" 2>&1)
                SPEED=$(echo "$OUTPUT" | grep -oP '[\d,]+(?= entries/s)' | tr -d ',' | head -1)
                [ -n "$SPEED" ] && SPEEDS="${SPEEDS:+$SPEEDS;}$SPEED"
            done

            if [ -n "$SPEEDS" ]; then
                STATS=$(calc_stats "$SPEEDS")
                AVG=$(echo    "$STATS" | cut -d',' -f1)
                MED=$(echo    "$STATS" | cut -d',' -f2)
                MIN=$(echo    "$STATS" | cut -d',' -f3)
                MAX=$(echo    "$STATS" | cut -d',' -f4)
                RUN_COUNT=$(echo "$SPEEDS" | tr ';' '\n' | wc -l | tr -d ' ')

                [ "$CSV" = true ] && echo "\"$DIR\",$T,$RUN_COUNT,$STATS,\"$SPEEDS\"" >> "$CSV_FILE"

                # Store for markdown
                MD_RESULTS+=("$DIR|$T|$AVG|$MED|$MIN|$MAX")

                if [ "$LIVE" = true ]; then
                    printf "  t=%-8s %14s %14s %14s %14s\n"                         "$T" "$(fmt_num "$AVG")" "$(fmt_num "$MED")" "$(fmt_num "$MIN")" "$(fmt_num "$MAX")"
                fi
            fi
        done
    done

    # Clear the progress line
    [ "$LIVE" = false ] && printf "\r%80s\r" ""

    # ── Write markdown summary ────────────────────────────────────────────────
    {
        echo "# ldx Benchmark Results"
        echo ""
        echo "**Date:** $DATE  "
        echo "**CPU:** $CPU  "
        echo "**OS:** $OS  "
        echo "**Cache:** $CACHE_TYPE  "
        echo "**Runs per combo:** $RUNS  "
        echo ""

        PREV_DIR=""
        for RESULT in "${MD_RESULTS[@]}"; do
            IFS='|' read -r D T AVG MED MIN MAX <<< "$RESULT"
            if [ "$D" != "$PREV_DIR" ]; then
                [ -n "$PREV_DIR" ] && echo ""
                echo "## $D"
                echo ""
                echo "| Threads | Avg (entries/s) | Median | Min | Max |"
                echo "|---------|----------------|--------|-----|-----|"
                PREV_DIR="$D"
            fi
            echo "| t=$T | $(fmt_num "$AVG") | $(fmt_num "$MED") | $(fmt_num "$MIN") | $(fmt_num "$MAX") |"
        done
        echo ""
    } > "$MD_FILE"

    # ── Final summary ─────────────────────────────────────────────────────────
    echo ""
    echo -e "${GREEN}${BOLD}Benchmark complete!${RESET}"
    echo -e "  ${CYAN}Report:${RESET} $MD_FILE"
    [ "$CSV" = true ] && echo -e "  ${CYAN}CSV:${RESET}    $CSV_FILE"
    echo ""
}

# ===========================================================================
# BUMP
# ===========================================================================

cmd_bump() {
    CARGO_TOML="Cargo.toml"
    VISION="project-vision.md"

    printf "  Checking Cargo.toml...     "
    CURRENT=$(grep '^version' "$CARGO_TOML" | head -1 | grep -oP '[\d]+\.[\d]+\.[\d]+')
    if [ -z "$CURRENT" ]; then
        echo -e "${RED}failed${RESET}"
        echo -e "${RED}Could not read version from $CARGO_TOML${RESET}"
        exit 1
    fi
    echo -e "${GREEN}v${CURRENT}${RESET}"

    printf "  Checking git tags...        "
    LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "none")
    echo -e "${GREEN}${LATEST_TAG}${RESET}"

    printf "  Reading git log...          "
    COMMITS=$(git log --oneline -5 2>/dev/null || echo "  (no git history)")
    echo -e "${GREEN}done${RESET}"

    IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT"

    printf "  Scanning project-vision.md... "
    NEXT_ROADMAP=""
    if [ -f "$VISION" ]; then
        while IFS= read -r line; do
            if echo "$line" | grep -qP '^### v[\d]+\.[\d]+\.[\d]+' && ! echo "$line" | grep -qE '✓|Shipped'; then
                NEXT_ROADMAP=$(echo "$line" | grep -oP '[\d]+\.[\d]+\.[\d]+')
                break
            fi
        done < "$VISION"
    fi

    SUGGESTED="${MAJOR}.${MINOR}.$((PATCH + 1))"
    [ -n "$NEXT_ROADMAP" ] && SUGGESTED="$NEXT_ROADMAP"

    VISION_NEXT=""
    if [ -f "$VISION" ] && [ -n "$NEXT_ROADMAP" ]; then
        IN_SECTION=false
        while IFS= read -r line; do
            if echo "$line" | grep -qP "^### v${NEXT_ROADMAP}"; then
                IN_SECTION=true; continue
            fi
            if $IN_SECTION; then
                echo "$line" | grep -qP '^###' && break
                echo "$line" | grep -qP '^\s*-' && VISION_NEXT="${VISION_NEXT}\n  ${line}"
            fi
        done < "$VISION"
    fi

    if [ -n "$NEXT_ROADMAP" ]; then
        echo -e "${GREEN}v${NEXT_ROADMAP} found${RESET}"
    else
        echo -e "${YELLOW}no unshipped entry${RESET}"
    fi

    echo ""
    echo -e "${CYAN}${BOLD}══════════════════════════════════════════════════════════════${RESET}"
    echo -e "${CYAN}${BOLD}               localdex / ldx Version Bumper${RESET}"
    echo -e "${CYAN}${BOLD}══════════════════════════════════════════════════════════════${RESET}"
    echo ""
    echo -e "  Current version : ${BOLD}${CURRENT}${RESET}"
    echo -e "  Latest git tag  : ${BOLD}${LATEST_TAG}${RESET}"
    echo ""
    echo -e "  ${BOLD}Recent commits (last 5):${RESET}"
    while IFS= read -r line; do
        echo -e "    ${CYAN}•${RESET} $line"
    done <<< "$COMMITS"
    echo ""

    if [ -n "$VISION_NEXT" ]; then
        echo -e "    ${CYAN}Planned for v${NEXT_ROADMAP}:${RESET}$VISION_NEXT"
    fi

    echo ""
    echo -e "  ${BOLD}Suggested → ${GREEN}${SUGGESTED}${RESET}"
    echo ""
    echo -e "${BOLD}What do you want to do?${RESET}"
    echo -e "  ${CYAN}1)${RESET} Accept suggested bump to ${SUGGESTED}"
    echo -e "  ${CYAN}2)${RESET} Enter custom version"
    echo -e "  ${CYAN}3)${RESET} Cancel"
    echo ""
    read -rp "> " CHOICE
    CHOICE=${CHOICE:-1}

    case "$CHOICE" in
        1) NEW_VERSION="$SUGGESTED" ;;
        2)
            read -rp "Enter version (e.g. 0.1.1): " NEW_VERSION
            if ! echo "$NEW_VERSION" | grep -qP '^\d+\.\d+\.\d+$'; then
                echo -e "${RED}Invalid version format. Use X.Y.Z${RESET}"
                exit 1
            fi ;;
        3) echo -e "${CYAN}Cancelled.${RESET}"; exit 0 ;;
        *) echo -e "${RED}Invalid choice.${RESET}"; exit 1 ;;
    esac

    echo ""
    echo -e "${YELLOW}Bump ${BOLD}${CURRENT}${RESET}${YELLOW} → ${BOLD}${NEW_VERSION}${RESET}${YELLOW} in $CARGO_TOML?${RESET}"
    read -rp "Confirm [y/N]: " CONFIRM
    [[ "$CONFIRM" =~ ^[Yy]$ ]] || { echo -e "${CYAN}Cancelled.${RESET}"; exit 0; }

    if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
        sed -i "s/^version = \"${CURRENT}\"/version = \"${NEW_VERSION}\"/" "$CARGO_TOML"
    else
        sed -i.bak "s/^version = \"${CURRENT}\"/version = \"${NEW_VERSION}\"/" "$CARGO_TOML"
        rm -f "${CARGO_TOML}.bak"
    fi

    echo ""
    echo -e "${GREEN}${BOLD}✓ Version bumped to ${NEW_VERSION}${RESET}"
    echo ""
    echo -e "${BOLD}Suggested next steps:${RESET}"
    echo -e "  ${CYAN}./scripts/dev.sh build${RESET}               # verify it compiles"
    echo -e "  ${CYAN}git add Cargo.toml${RESET}"
    echo -e "  ${CYAN}git commit -m \"v${NEW_VERSION} - <description>\"${RESET}"
    echo -e "  ${CYAN}git tag v${NEW_VERSION}${RESET}"
    echo -e "  ${CYAN}git push && git push --tags${RESET}"
    echo ""
}

# ─── Command dispatch ─────────────────────────────────────────────────────────
COMMAND="${1:-}"
shift || true

case "$COMMAND" in
    build)             cmd_build "$@" ;;
    benchmark|bench)   cmd_benchmark "$@" ;;
    bump)              cmd_bump "$@" ;;
    help|--help|-h|"") show_help ;;
    *)
        echo -e "${RED}Unknown command: $COMMAND${RESET}"
        show_help
        exit 1 ;;
esac
