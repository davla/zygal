#!/usr/bin/env zsh

ZYGAL_THEME_ROOT=${${(%):-%x}:h:h}

source "$ZYGAL_THEME_ROOT/deps/zsh-async/async.zsh"
source "$ZYGAL_THEME_ROOT/lib/git.sh"

zygal_append_git() {
    PROMPT="${ZYGAL_PRE_VCS}${3}${ZYGAL_POST_VCS}"
    zle reset-prompt
}

zygal_async_init() {
    async_init

    async_start_worker zygal_git_base
    async_register_callback zygal_git_base zygal_append_git

    async_start_worker zygal_git_remote
    async_register_callback zygal_git_remote zygal_append_git
}

zygal_async() {
    async_job zygal_git_base zygal_git_info "$ZYGAL_VCS"
    async_job zygal_git_remote zygal_git_remote "$ZYGAL_VCS"
}
