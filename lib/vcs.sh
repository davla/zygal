#!/bin/sh

#shellcheck disable=2039
#shellcheck disable=2154

[ -n "$ZYGAL_THEME_ROOT" ] && ROOT_DEFINED=true || ROOT_DEFINED=false
$ROOT_DEFINED || {
    if [ -n "$BASH_VERSION" ]; then
        THIS_FILE="${BASH_SOURCES[0]}"
    elif [ -n "$ZSH_VERSION" ]; then
        THIS_FILE="${(%):-%x}"
    fi

    ZYGAL_THEME_ROOT="$(readlink -f "$THIS_FILE" | xargs dirname \
        | xargs dirname)"
    unset THIS_FILE
}

. "$ZYGAL_THEME_ROOT/lib/git.sh"
. "$ZYGAL_THEME_ROOT/lib/hg.sh"

zygal_vcs_info() {
    zygal_git_prompt_info "$1"
    zygal_hg_prompt_info "$1"
}

$ROOT_DEFINED || unset ZYGAL_THEME_ROOT
unset ROOT_DEFINED
