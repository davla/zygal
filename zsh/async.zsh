#!/usr/bin/env zsh

ZYGAL_THEME_ROOT=${${(%):-%x}:h:h}

source "$ZYGAL_THEME_ROOT/deps/zsh-async/async.zsh"
source "$ZYGAL_THEME_ROOT/lib/git.sh"

zygal_append_git() {
    if [ -n "$3" ]; then
        PROMPT="${ZYGAL_PRE_VCS}${3}${ZYGAL_POST_VCS}"
        zle reset-prompt
    fi
}

zygal_async_init() {
    async_init

    async_start_worker zygal_git_base
    async_register_callback zygal_git_base zygal_append_git

    if $ZYGAL_ENABLE_VCS_REMOTE; then
        async_start_worker zygal_git_remote
        async_register_callback zygal_git_remote zygal_append_git
    fi
}

zygal_async() {
    local PWD_CMD="cd $PWD"

    async_worker_eval zygal_git_base "$PWD_CMD"
    async_job zygal_git_base zygal_git_info "$ZYGAL_VCS"

    if $ZYGAL_ENABLE_VCS_REMOTE; then
        async_worker_eval zygal_git_remote "$PWD_CMD"
        async_job zygal_git_remote zygal_git_remote "$ZYGAL_VCS"
    fi
}
