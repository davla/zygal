#!/bin/sh

if [ -n "$BASH_VERSION" ]; then
    THIS_SCRIPT="${BASH_SOURCES[0]}"
elif [ -n "$ZSH_VERSION" ]; then
    THIS_SCRIPT="${(%):-%x}"
fi

ZYGAL_VCS_REMOTE="${ZYGAL_VCS_REMOTE-true}"
ZYGAL_VCS_REMOTE_SYNC_TRIGGER="${ZYGAL_VCS_REMOTE_REFRESH_COUNT:-10}"
ZYGAL_VCS_REMOTE_COUNT=-1

ZYGAL_THEME_ROOT="$(dirname "$THIS_SCRIPT" | xargs -i readlink -f '{}/..')"

source "$ZYGAL_THEME_ROOT/lib/git.sh"
source "$ZYGAL_THEME_ROOT/lib/hg.sh"

zygal_vcs_info() {
    zygal_git_prompt_info "$1"
    zygal_hg_prompt_info "$1"
}

zygal_vcs_info_remote() {
    zygal_git_sync_remote

    zygal_vcs_info "$1"
}
