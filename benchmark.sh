#!/usr/bin/env bash
# benchmark.sh — Benchmark ldx across directories and thread counts
# Supports: Linux, macOS, Windows (Git Bash)
# Usage: ./benchmark.sh [options]

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

# ─── Defaults ─────────────────────────────────────────────────────────────────
RUNS=10
CACHE_TYPE="warm"
THREAD_LIST="1,2,4,6,8,10,12,14,16"
CUSTOM_DIRS=""
CUSTOM_OUT=""

# ─── Parse arguments ──────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
    case "$1" in
        --runs)
            RUNS="$2"
            shift 2 ;;
        --cold)
            CACHE_TYPE="cold"
            shift ;;
        --warm)
            CACHE_TYPE="warm"
            shift ;;
        --threads)
            THREAD_LIST="$2"
            shift 2 ;;
        --dirs)
            CUSTOM_DIRS="$2"
            shift 2 ;;
        --out)
            CUSTOM_OUT="$2"
            shift 2 ;;
        --help)
            echo "Usage: ./benchmark.sh [options]"
            echo ""
            echo "Options:"
            echo "  --runs N           Number of runs per combination (default: 10)"
            echo "  --cold             Label output as cold cache run"
            echo "  --warm             Label output as warm cache run (default)"
            echo "  --threads LIST     Comma-separated thread counts (default: 1,2,4,6,8,10,12,14,16)"
            echo "  --dirs LIST        Comma-separated directories to test"
            echo "  --out FILE         Custom output filename"
            echo "  --help             Show this help message"
            echo ""
            echo "Examples:"
            echo "  ./benchmark.sh --runs 20 --cold"
            echo "  ./benchmark.sh --runs 5 --threads 1,4,8,16"
            echo "  ./benchmark.sh --dirs /home,/usr --runs 10"
            exit 0 ;;
        *)
            echo -e "${RED}Unknown argument: $1${RESET}"
            echo "Run ./benchmark.sh --help for usage"
            exit 1 ;;
    esac
done

# ─── Check for ldx ────────────────────────────────────────────────────────────
if ! command -v ldx &> /dev/null; then
    echo -e "${RED}ldx not found in PATH. Run build.sh first.${RESET}"
    exit 1
fi

# ─── Default directories per OS ───────────────────────────────────────────────
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

# ─── Parse thread list ────────────────────────────────────────────────────────
IFS=',' read -ra THREADS <<< "$THREAD_LIST"

# ─── CPU name for filename ────────────────────────────────────────────────────
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
if [ -z "$CPU_SHORT" ]; then
    CPU_SHORT="unknown-cpu"
fi

# ─── Output filename ──────────────────────────────────────────────────────────
DATE=$(date +%Y-%m-%d)
if [ -n "$CUSTOM_OUT" ]; then
    OUT_FILE="$CUSTOM_OUT"
else
    OUT_FILE="benchmark_${CACHE_TYPE}_${RUNS}runs_${CPU_SHORT}_${DATE}.csv"
fi

# ─── Calculate totals ─────────────────────────────────────────────────────────
TOTAL_COMBOS=$(( ${#DIRS[@]} * ${#THREADS[@]} ))
TOTAL_RUNS=$(( TOTAL_COMBOS * RUNS ))

# ─── Header ───────────────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}${BOLD}ldx Benchmark Script${RESET}"
echo -e "${CYAN}OS: $OS | Cache: $CACHE_TYPE | Runs per combo: $RUNS${RESET}"
echo -e "${CYAN}CPU: $CPU${RESET}"
echo -e "${CYAN}Directories: ${#DIRS[@]} | Thread configs: ${#THREADS[@]} | Total runs: $TOTAL_RUNS${RESET}"
echo -e "${CYAN}Output: $OUT_FILE${RESET}"
echo ""

# ─── Write CSV header ─────────────────────────────────────────────────────────
echo "Directory,Threads,Runs,Avg,Median,Min,Max,AllSpeeds" > "$OUT_FILE"

# ─── Helper: calculate stats with awk ─────────────────────────────────────────
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

# ─── Main benchmark loop ──────────────────────────────────────────────────────
CURRENT_RUN=0

for DIR in "${DIRS[@]}"; do
    for T in "${THREADS[@]}"; do
        SPEEDS=""

        for (( i=1; i<=RUNS; i++ )); do
            CURRENT_RUN=$(( CURRENT_RUN + 1 ))
            printf "\r${CYAN}[%d/%d]${RESET} Dir: %-30s Threads: %-3s Run: %d/%d" \
                "$CURRENT_RUN" "$TOTAL_RUNS" "$DIR" "$T" "$i" "$RUNS"

            OUTPUT=$(ldx -a -q -S -d "$DIR" -t "$T" 2>&1)
            SPEED=$(echo "$OUTPUT" | grep -oP '[\d,]+(?= entries/s)' | tr -d ',' | head -1)

            if [ -n "$SPEED" ]; then
                if [ -z "$SPEEDS" ]; then
                    SPEEDS="$SPEED"
                else
                    SPEEDS="$SPEEDS;$SPEED"
                fi
            fi
        done

        if [ -n "$SPEEDS" ]; then
            STATS=$(calc_stats "$SPEEDS")
            RUN_COUNT=$(echo "$SPEEDS" | tr ';' '\n' | wc -l | tr -d ' ')
            echo "\"$DIR\",$T,$RUN_COUNT,$STATS,\"$SPEEDS\"" >> "$OUT_FILE"

            AVG=$(echo "$STATS" | cut -d',' -f1)
            printf "\r${GREEN}+${RESET} %-30s t=%-3s avg=%s/s%50s\n" \
                "$DIR" "$T" "$AVG" ""
        fi
    done
done

# ─── Done ─────────────────────────────────────────────────────────────────────
echo ""
echo -e "${GREEN}${BOLD}Benchmark complete!${RESET}"
echo -e "${CYAN}Results saved to: ${BOLD}${OUT_FILE}${RESET}"
echo ""
