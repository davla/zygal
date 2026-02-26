#!/usr/bin/env sh
# shellcheck disable=SC2059

# This script is mostly a wrapper around cargo build, turning some command-line
# parameters to environment variables. For more information, read the help text

########################################
# Variables
########################################

COLORSCHEME='orange'
CONFIG='config'
OUTPUT_IN_PATH="$HOME/bin /usr/local/bin /usr/bin"

BLUE='\033[34m'
GREEN='\033[32m'
YELLOW='\033[33m'

BOLD='\033[1m'
RESET='\033[0m'

HELP_TEXT="\
${BLUE}cargo build --release$RESET wrapper for Zygal.

${BOLD}Usage:$RESET

${BLUE}./zygal-build.sh$RESET [${GREEN}--colorscheme$RESET COLORSCHEME] [${GREEN}--zygal-config$RESET CONFIG] [${GREEN}--zygal-output$RESET OUTPUT_PATH]
${BLUE}./zygal-build.sh$RESET [${GREEN}--help$RESET|${GREEN}-H$RESET]

${BOLD}Description:$RESET

Build the ${BLUE}zygal-prompt$RESET binary and copy it to ${YELLOW}ZYGAL_PROMPT$RESET or a directory on ${YELLOW}PATH$RESET.

${BLUE}cargo build --release$RESET is invoked with some environment variables populated from command-line parameters.
The $GREEN--colorscheme$RESET and $GREEN--zygal-config$RESET parameters are passed in as the ${YELLOW}ZYGAL_COLORSCHEME$RESET and ${YELLOW}ZYGAL_CONFIG$RESET environment variables respectively.
The built ${BLUE}zygal-prompt$RESET binary is copied to the path in ${YELLOW}ZYGAL_PROMPT$RESET, which can be overridden via the $GREEN--zygal-output$RESET parameter. If neither ${YELLOW}ZYGAL_PROMPT$RESET nor $GREEN--zygal-output$RESET are given, ${BLUE}zygal-prompt$RESET is copied to the first directory, in this order, that is in ${YELLOW}PATH$RESET: $OUTPUT_IN_PATH.

${BOLD}Arguments$RESET:

    $BOLD--colorscheme COLORSCHEME, --zygal-colorscheme COLORSCHEME$RESET
        The Zygal color scheme to be used. ${BOLD}Optional$RESET, defaults to '$GREEN$COLORSCHEME$RESET'.

    $BOLD--help, -H$RESET
        Show this help text.

    $BOLD--zygal-output PATH$RESET
        Override the path the built ${BLUE}zygal-prompt$RESET binary is copied to.
        ${BOLD}Optional$RESET, defaults first to the path in the ${YELLOW}ZYGAL_PROMPT$RESET environment variable (current: '$GREEN$ZYGAL_PROMPT$RESET'),
        or to the first of these directories found in ${YELLOW}PATH$RESET: $OUTPUT_IN_PATH.

    $BOLD--zygal-config CONFIG_FILE$RESET
        The Zygal configuration file. ${BOLD}Optional$RESET, defaults to '$GREEN$CONFIG$RESET'.
"

########################################
# Input processing
########################################

OUTPUT="$ZYGAL_PROMPT"
while [ "$#" -gt 0 ]; do
    case "$1" in
        '--colorscheme'|'--zygal-colorscheme')
            COLORSCHEME="$2"
            shift
            ;;

        '--help'|'-H')
            # shellcheck disable=SC2059
            printf "$HELP_TEXT"
            exit
            ;;

        '--zygal-config')
            CONFIG="$2"
            shift
            ;;

        '--zygal-output')
            OUTPUT="$2"
            shift
            ;;

        *)
            echo >&2 "Unknown parameter: '$1'"
            exit 63
            ;;
    esac
    shift
done

[ -z "$OUTPUT" ] && {
    for OUTPUT_DIR in $OUTPUT_IN_PATH; do
        echo "$PATH" | grep --quiet "$OUTPUT_DIR" && {
            OUTPUT="$OUTPUT_DIR/zygal-prompt"
            break
        }
    done
}

[ -z "$OUTPUT" ] && {
    printf >&2 "Can't find an output path. Fill the ${YELLOW}ZYGAL_PROMPT$RESET environment variable, use the $GREEN--zygal-output$RESET argument, or make sure one of these directory is in ${YELLOW}PATH$RESET: $OUTPUT_IN_PATH\n"
    exit 62
}

########################################
# Main
########################################

# This doesn't work if this script is sourced
cd "$(dirname "$0")/zygal-prompt" || exit 64

printf "${GREEN}[INFO]$RESET Build ${BLUE}zygal-prompt$RESET...\n"
ZYGAL_COLORSCHEME="$COLORSCHEME" ZYGAL_CONFIG="$CONFIG" cargo build --release

printf "${GREEN}[INFO]$RESET Copy ${BLUE}zygal-prompt$RESET to $GREEN$OUTPUT$RESET\n"
if [ -w "$(dirname "$OUTPUT")" ]; then
    install -D --mode 755 target/release/zygal-prompt "$OUTPUT"
else
    sudo install -D --mode 755 target/release/zygal-prompt "$OUTPUT"
fi
