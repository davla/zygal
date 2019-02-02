#!/bin/sh

ZYGAL_ASYNC="${ZYGAL_ASYNC:-remote}"
ZYGAL_COLORSCHEME=${ZYGAL_COLORSCHEME:-orange}

ZYGAL_ENABLE_VCS_REMOTE="${ZYGAL_ENABLE_VCS_REMOTE-true}"
$ZYGAL_ENABLE_VCS_REMOTE && {
    ZYGAL_VCS_REMOTE_SYNC_TRIGGER="${ZYGAL_VCS_REMOTE_REFRESH_COUNT:-10}"
    ZYGAL_VCS_REMOTE_COUNT=-1
}

ZYGAL_GIT_PROMPT_PATH="${ZYGAL_GIT_PROMPT_PATH\
:-/usr/lib/git-core/git-sh-prompt}"

[ $ZYGAL_ASYNC != 'none' ] && {
    if [ -n "$BASH_VERSION" ]; then
        THIS_FILE="${BASH_SOURCES[0]}"
    elif [ -n "$ZSH_VERSION" ]; then
        THIS_FILE="${(%):-%x}"
    fi

    ZYGAL_THEME_ROOT="$(readlink -f "$THIS_FILE" | xargs dirname \
        | xargs dirname)"
    unset THIS_FILE
    ZYGAL_ZSH_ASYNC_PATH="${ZYGAL_ZSH_ASYNC_PATH:-$ZYGAL_THEME_ROOT\
/deps/zsh-async}"
}

if [ "$ZYGAL_ASYNC" = 'remote' ] && ! $ZYGAL_ENABLE_VCS_REMOTE; then
    echo -n >&2 'Remote features disabled (ZYGAL_ENABLE_VCS_REMOTE) but '
    echo >&2 "ZYGAL_ASYNC set to to 'remote'"
    false
else
    true
fi
