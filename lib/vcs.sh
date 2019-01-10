#!/usr/bin/env sh

PARENT_DIR="$(print -P %x | xargs dirname | xargs readlink -f)"

. "$PARENT_DIR/git.sh"
. "$PARENT_DIR/hg.sh"

vcs_info() {
    ZYGAL_GIT_INFO="$(git_info "$1")"
    ZYGAL_HG_INFO="$(hg_info "$2")"

    echo $ZYGAL_GIT_INFO $ZYGAL_HG_INFO

    unset ZYGAL_GIT_INFO ZYGAL_HG_INFO
}
