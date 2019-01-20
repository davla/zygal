#!/bin/sh

if [ -n "$BASH_VERSION" ]; then
    THIS_SCRIPT="${BASH_SOURCES[0]}"
elif [ -n "$ZSH_VERSION" ]; then
    THIS_SCRIPT="${(%):-%x}"
fi

ZYGAL_THEME_ROOT="$(dirname "$THIS_SCRIPT" | xargs -i readlink -f '{}/..')"

source "$ZYGAL_THEME_ROOT/lib/git.sh"
source "$ZYGAL_THEME_ROOT/lib/hg.sh"

zygal_vcs_info() {
    zygal_git_info "$1"
    zygal_hg_info "$1"
}
