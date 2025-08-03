#!/bin/bash
RC='\e[0m'
BOLD='\e[1m'
RED='\e[31m'
GREEN='\e[32m'
YELLOW='\e[33m'
BLUE='\e[34m'
GREY='\e[97m'

print() {
    local tag="${1}"
    local message="$2"
    local level="${3:-info}"

    local tag_text=""
    if [ -n "$tag" ]; then
        tag_text="[${BOLD}$tag${RC}]"
    fi

    case "$level" in
    error)
        echo -e "${tag_text} ${BOLD}${RED}ERROR${RC} - $message"
        ;;
    warn)
        echo -e "${tag_text} ${BOLD}${YELLOW}WARN${RC} - $message"
        ;;
    none)
        echo -e "${tag_text} $message"
        ;;
    *)
        echo -e "${tag_text} ${BOLD}${GREEN}INFO${RC} - $message"
        ;;
    esac
}

spinner() {
    local message="$1"
    # These are nice-looking braille characters for the spinner
    local chars="⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
    while :; do
        for ((i = 0; i < ${#chars}; i++)); do
            # Use printf with \r to return to the beginning of the line
            printf "\r${GREY}%s %s${RC}" "${chars:$i:1}" "$message"
            sleep 0.1
        done
    done
}

wait_for_http() {
    local url=$1
    local timeout=${2:-30}
    local count=0

    print "Health Check" "Waiting for $url to respond..." debug

    while [ $count -lt $timeout ]; do
        if curl -s --connect-timeout 2 --max-time 5 "$url" >/dev/null 2>&1; then
            return 0
        fi

        sleep 1
        count=$((count + 1))

        if [ $((count % 10)) -eq 0 ]; then
            print "Health Check" "Still waiting for HTTP response... (${count}s/${timeout}s)" debug
        fi
    done
    return 1
}

check_port() {
    local port=$1
    if command -v lsof >/dev/null 2>&1; then
        lsof -ti:"$port" -sTCP:LISTEN >/dev/null 2>&1
    elif command -v netstat >/dev/null 2>&1; then
        netstat -tlnp 2>/dev/null | grep -q ":$port "
    elif command -v ss >/dev/null 2>&1; then
        ss -tlnp 2>/dev/null | grep -q ":$port "
    else
        return 1
    fi
}

install_deps() {
    local dir="$1"
    local context="$2"
    local title="Node Packages"

    if [ ! -f "$dir/package.json" ]; then
        return 0
    fi

    cd "$dir"

    if [ ! -d "node_modules" ]; then
        print "$title" "Installing $context dependencies..."
        npm install
    elif [ "package.json" -nt "node_modules" ]; then
        print "$title" "Updating $context dependencies..."
        npm install
    else
        print "$title" "$context dependencies are up to date"
    fi

    cd - >/dev/null
}
