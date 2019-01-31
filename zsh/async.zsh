#!/usr/bin/env zsh

THIS_FILE="$(readlink -f "${(%):-%x}")"
ZYGAL_THEME_ROOT="${THIS_FILE:h:h}"
unset THIS_FILE

source "$ZYGAL_THEME_ROOT/lib/config.sh"
source "$ZYGAL_ZSH_ASYNC_PATH/async.zsh"

zygal_append_vcs() {
    [ -n "$3" ] && {
        PROMPT="${ZYGAL_PRE_VCS}${3}${ZYGAL_POST_VCS}"
        zle reset-prompt
    }
}

zygal_append_vcs_and_stop() {
    zygal_append_vcs "$@"
    async_stop_worker "$ZYGAL_WORKER_NAME"
    unset ZYGAL_WORKER_NAME
}

zygal_async_init() {
    async_init

    [ "$ZYGAL_ASYNC" = 'all' ] && {
        async_start_worker zygal_worker_vcs_base
        async_register_callback zygal_worker_vcs_base zygal_append_vcs
    }
}

zygal_async() {
    [ "$ZYGAL_ASYNC" = 'all' ] && {
        async_worker_eval zygal_worker_vcs_base "cd $PWD"
        async_job zygal_worker_vcs_base zygal_vcs_info "$ZYGAL_VCS_FORMAT"
    }

    $ZYGAL_ENABLE_VCS_REMOTE && [ "$ZYGAL_VCS_REMOTE_COUNT" -eq 0 ] && {
        typeset -g ZYGAL_WORKER_NAME='zygal_worker_vcs_remote'

        async_start_worker "$ZYGAL_WORKER_NAME"
        async_register_callback "$ZYGAL_WORKER_NAME" \
            zygal_append_vcs_and_stop
        async_job "$ZYGAL_WORKER_NAME" zygal_vcs_info_remote \
            "$ZYGAL_VCS_FORMAT"
    }
}
