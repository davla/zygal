#!/usr/bin/env zsh

[ -n "$ZYGAL_THEME_ROOT" ] && ROOT_DEFINED=true || ROOT_DEFINED=false
$ROOT_DEFINED || {
    THIS_FILE="$(readlink -f "${(%):-%x}")"
    ZYGAL_THEME_ROOT="${THIS_FILE:h:h}"
    unset THIS_FILE
}

source "$ZYGAL_THEME_ROOT/lib/config.sh"
source "$ZYGAL_THEME_ROOT/lib/vcs.sh"

[ $? -eq 0 ] && {
    source "$ZYGAL_ZSH_ASYNC_PATH/async.zsh"

    zygal_append_vcs() {
        [ -n "$3" ] && {
            PROMPT="${ZYGAL_PRE_VCS}${3}${ZYGAL_POST_VCS}"
            [ "$6" = 0 ] && zle reset-prompt
        }

        ZYGAL_ASYNC_RUNNING_COUNT=$(( ZYGAL_ASYNC_RUNNING_COUNT - 1 ))
        [ "$ZYGAL_ASYNC_RUNNING_COUNT" -eq 0 ] \
            && async_stop_worker zygal_async_worker
    }

    zygal_async() {
        [ "$ZYGAL_ASYNC" = 'all' ] && {
            async_start_worker zygal_async_worker
            async_register_callback zygal_async_worker zygal_append_vcs
        }

        [ "$ZYGAL_ASYNC" = 'all' ] && {
            ZYGAL_ASYNC_RUNNING_COUNT=$(( ZYGAL_ASYNC_RUNNING_COUNT + 1 ))
            async_job zygal_async_worker zygal_vcs_info "$ZYGAL_VCS_FORMAT"
        }
    }
}

$ROOT_DEFINED || unset ZYGAL_THEME_ROOT
unset ROOT_DEFINED
