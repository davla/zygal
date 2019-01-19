#!/bin/sh

if [ -n "$BASH_VERSION" ]; then
    THIS_SCRIPT="${BASH_SOURCES[0]}"
elif [ -n "$ZSH_VERSION" ]; then
    THIS_SCRIPT="${(%):-%x}"
fi

PARENT_DIR="$(dirname "$THIS_SCRIPT" | xargs readlink -f)"

source "$PARENT_DIR/git.sh"
source "$PARENT_DIR/hg.sh"

zygal_vcs_info() {
    zygal_git_info "$1"
    zygal_hg_info "$1"
}
