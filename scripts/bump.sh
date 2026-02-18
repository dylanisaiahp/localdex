#!/usr/bin/env bash
# bump.sh — Suggest and apply version bumps for localdex
# Usage: ./scripts/bump.sh

set -e

# ─── Colors ───────────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
RESET='\033[0m'

CARGO_TOML="Cargo.toml"
VISION="project-vision.md"

# ─── Check we're in the repo root ─────────────────────────────────────────────
if [ ! -f "$CARGO_TOML" ]; then
    echo -e "${RED}Error: $CARGO_TOML not found. Run this from the repo root.${RESET}"
    exit 1
fi

# ─── Read current version ─────────────────────────────────────────────────────
CURRENT=$(grep '^version' "$CARGO_TOML" | head -1 | grep -oP '[\d]+\.[\d]+\.[\d]+')
if [ -z "$CURRENT" ]; then
    echo -e "${RED}Could not read version from $CARGO_TOML${RESET}"
    exit 1
fi

# ─── Latest git tag ───────────────────────────────────────────────────────────
LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "none")

# ─── Recent commits ───────────────────────────────────────────────────────────
COMMITS=$(git log --oneline -5 2>/dev/null || echo "  (no git history)")

# ─── Suggest next version ─────────────────────────────────────────────────────
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT"

# Find the next roadmap entry in project-vision.md
NEXT_ROADMAP=""
NEXT_DESC=""
if [ -f "$VISION" ]; then
    # Find the first roadmap entry that is NOT marked as shipped (✓)
    while IFS= read -r line; do
        if echo "$line" | grep -qP '^### v[\d]+\.[\d]+\.[\d]+' && ! echo "$line" | grep -q '✓'; then
            NEXT_ROADMAP=$(echo "$line" | grep -oP '[\d]+\.[\d]+\.[\d]+')
            # Grab the next non-empty line as description
            break
        fi
    done < "$VISION"
fi

# Default suggestion: bump patch
SUGGESTED="${MAJOR}.${MINOR}.$((PATCH + 1))"
# Use roadmap version if it's a legitimate next step
if [ -n "$NEXT_ROADMAP" ]; then
    SUGGESTED="$NEXT_ROADMAP"
fi

# ─── Scan vision for what's coming ────────────────────────────────────────────
VISION_NEXT=""
if [ -f "$VISION" ] && [ -n "$NEXT_ROADMAP" ]; then
    # Pull bullet points under the next roadmap version heading
    IN_SECTION=false
    while IFS= read -r line; do
        if echo "$line" | grep -qP "^### v${NEXT_ROADMAP}"; then
            IN_SECTION=true
            continue
        fi
        if $IN_SECTION; then
            if echo "$line" | grep -qP '^###'; then
                break
            fi
            if echo "$line" | grep -qP '^\s*-'; then
                VISION_NEXT="${VISION_NEXT}\n  ${line}"
            fi
        fi
    done < "$VISION"
fi

# ─── Print header ─────────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}${BOLD}══════════════════════════════════════════════════════════════${RESET}"
echo -e "${CYAN}${BOLD}               localdex / prx Version Bumper${RESET}"
echo -e "${CYAN}${BOLD}══════════════════════════════════════════════════════════════${RESET}"
echo ""
echo -e "  Current version in $CARGO_TOML : ${BOLD}${CURRENT}${RESET}"
echo -e "  Latest git tag                : ${BOLD}${LATEST_TAG}${RESET}"
echo ""
echo -e "  ${BOLD}Recent commits (last 5):${RESET}"
while IFS= read -r line; do
    echo -e "    ${CYAN}•${RESET} $line"
done <<< "$COMMITS"
echo ""

if [ -f "$VISION" ]; then
    echo -e "  ${BOLD}Scanning $VISION...${RESET}"
    if [ -n "$NEXT_ROADMAP" ]; then
        echo -e "    ${GREEN}✓${RESET} Next roadmap entry: v${NEXT_ROADMAP}"
        if [ -n "$VISION_NEXT" ]; then
            echo -e "    ${CYAN}Planned for v${NEXT_ROADMAP}:${RESET}"
            echo -e "$VISION_NEXT"
        fi
    else
        echo -e "    ${YELLOW}No unshipped roadmap entry found — defaulting to patch bump${RESET}"
    fi
else
    echo -e "  ${YELLOW}$VISION not found — defaulting to patch bump${RESET}"
fi

echo ""
echo -e "  ${BOLD}Suggested bump → ${GREEN}${SUGGESTED}${RESET}"
echo ""

# ─── Prompt ───────────────────────────────────────────────────────────────────
echo -e "${BOLD}What do you want to do?${RESET}"
echo -e "  ${CYAN}1)${RESET} Accept suggested bump to ${SUGGESTED}"
echo -e "  ${CYAN}2)${RESET} Enter custom version"
echo -e "  ${CYAN}3)${RESET} Cancel"
echo ""
read -rp "> " CHOICE
CHOICE=${CHOICE:-1}

case "$CHOICE" in
    1)
        NEW_VERSION="$SUGGESTED"
        ;;
    2)
        read -rp "Enter version (e.g. 0.0.9): " NEW_VERSION
        if ! echo "$NEW_VERSION" | grep -qP '^\d+\.\d+\.\d+$'; then
            echo -e "${RED}Invalid version format. Use X.Y.Z${RESET}"
            exit 1
        fi
        ;;
    3)
        echo -e "${CYAN}Cancelled.${RESET}"
        exit 0
        ;;
    *)
        echo -e "${RED}Invalid choice.${RESET}"
        exit 1
        ;;
esac

# ─── Confirm ──────────────────────────────────────────────────────────────────
echo ""
echo -e "${YELLOW}Bump ${BOLD}${CURRENT}${RESET}${YELLOW} → ${BOLD}${NEW_VERSION}${RESET}${YELLOW} in $CARGO_TOML?${RESET}"
read -rp "Confirm [y/N]: " CONFIRM
if ! [[ "$CONFIRM" =~ ^[Yy]$ ]]; then
    echo -e "${CYAN}Cancelled.${RESET}"
    exit 0
fi

# ─── Apply ────────────────────────────────────────────────────────────────────
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
    # Git Bash on Windows — sed -i needs no backup extension
    sed -i "s/^version = \"${CURRENT}\"/version = \"${NEW_VERSION}\"/" "$CARGO_TOML"
else
    sed -i.bak "s/^version = \"${CURRENT}\"/version = \"${NEW_VERSION}\"/" "$CARGO_TOML"
    rm -f "${CARGO_TOML}.bak"
fi

echo ""
echo -e "${GREEN}${BOLD}✓ Version bumped to ${NEW_VERSION} in $CARGO_TOML${RESET}"
echo ""
echo -e "${BOLD}Suggested next steps:${RESET}"
echo -e "  ${CYAN}cargo build --release${RESET}                    # verify it compiles"
echo -e "  ${CYAN}git add Cargo.toml${RESET}"
echo -e "  ${CYAN}git commit -m \"v${NEW_VERSION} - <description>\"${RESET}"
echo -e "  ${CYAN}git tag v${NEW_VERSION}${RESET}"
echo -e "  ${CYAN}git push && git push --tags${RESET}"
echo ""
