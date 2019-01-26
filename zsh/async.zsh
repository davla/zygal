#!/usr/bin/env zsh

ZYGAL_THEME_ROOT=${${(%):-%x}:h:h}

source "$ZYGAL_THEME_ROOT/deps/zsh-async/async.zsh"
source "$ZYGAL_THEME_ROOT/lib/git.sh"

zygal_append_vcs() {
    if [ -n "$3" ]; then
        PROMPT="${ZYGAL_PRE_VCS}${3}${ZYGAL_POST_VCS}"
        zle reset-prompt
    fi
}

zygal_append_vcs_and_stop() {
    zygal_append_vcs "$@"
    async_stop_worker "$ZYGAL_WORKER_NAME"
    unset ZYGAL_WORKER_NAME
}

zygal_async_init() {
    async_init

    [ "$ZYGAL_ASYNC" = 'all' ] && {
        async_start_worker zygal_vcs_base
        async_register_callback zygal_vcs_base zygal_append_vcs
    }
}

zygal_async() {
    local PWD_CMD="cd $PWD"

    [ "$ZYGAL_ASYNC" = 'all' ] && {
        async_worker_eval zygal_vcs_base "$PWD_CMD"
        async_job zygal_vcs_base zygal_vcs_info "$ZYGAL_VCS_FORMAT"
    }

    if $ZYGAL_ENABLE_VCS_REMOTE; then
        case "$ZYGAL_ASYNC" in
            'all'|'remote')
                [ "$ZYGAL_VCS_REMOTE_COUNT" -eq 0 ] && {
                    typeset -g ZYGAL_WORKER_NAME='zygal_vcs_remote'

                    async_start_worker "$ZYGAL_WORKER_NAME"
                    async_register_callback "$ZYGAL_WORKER_NAME" \
                        zygal_append_vcs_and_stop

                    async_worker_eval "$ZYGAL_WORKER_NAME" "$PWD_CMD"
                    async_job "$ZYGAL_WORKER_NAME" zygal_vcs_info_remote \
                        "$ZYGAL_VCS_FORMAT"
                }

                ;;
        esac
    fi
}
