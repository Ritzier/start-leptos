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
