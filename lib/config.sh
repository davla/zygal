#!/bin/sh

#shellcheck disable=2034
#shellcheck disable=2039
#shellcheck disable=2154

ZYGAL_ASYNC="${ZYGAL_ASYNC:-remote}"
ZYGAL_COLORSCHEME=${ZYGAL_COLORSCHEME:-orange}

ZYGAL_ENABLE_VCS_REMOTE="${ZYGAL_ENABLE_VCS_REMOTE-true}"
$ZYGAL_ENABLE_VCS_REMOTE && {
    ZYGAL_VCS_REMOTE_REFRESH_COUNT="${ZYGAL_VCS_REMOTE_REFRESH_COUNT:-10}"
    ZYGAL_VCS_REMOTE_COUNT=-1
}

[ -z "$ZYGAL_GIT_PROMPT_PATH" ] && {
    ZYGAL_GIT_PROMPT_FILES='/usr/lib/git-core/git-sh-prompt
/usr/share/git/completion/git-prompt.sh'

    TMP_FILE="$(mktemp)"
    echo "$ZYGAL_GIT_PROMPT_FILES" > "$TMP_FILE"

    while read ZYGAL_GIT_PROMPT_FILE; do
        if [ -f "$ZYGAL_GIT_PROMPT_FILE" ]; then
            ZYGAL_GIT_PROMPT_PATH="$ZYGAL_GIT_PROMPT_FILE"
            break
        fi
    done < "$TMP_FILE"

    rm "$TMP_FILE"
    unset ZYGAL_GIT_PROMPT_FILES ZYGAL_GIT_PROMPT_FILES
}
[ -z "$ZYGAL_GIT_PROMPT_PATH" ] && {
    printf >&2 'No git prompt file found to source. Make sure you defined '
    echo >&2 'ZYGAL_GIT_PROMPT_PATH'
    exit
}

[ $ZYGAL_ASYNC != 'none' ] && {
    [ -n "$ZYGAL_THEME_ROOT" ] && ROOT_DEFINED=true || ROOT_DEFINED=false
    $ROOT_DEFINED || {
        if [ -n "$BASH_VERSION" ]; then
            THIS_FILE="${BASH_SOURCES[0]}"
        elif [ -n "$ZSH_VERSION" ]; then
            THIS_FILE="${(%):-%x}"
        fi
        ZYGAL_THEME_ROOT="$(readlink -f "$THIS_FILE" | xargs dirname \
            | xargs dirname)"
        unset THIS_FILE
    }

    ZYGAL_ZSH_ASYNC_PATH="${ZYGAL_ZSH_ASYNC_PATH:-$ZYGAL_THEME_ROOT\
/deps/zsh-async}"

    $ROOT_DEFINED || unset ZYGAL_THEME_ROOT
    unset ROOT_DEFINED
}

if [ "$ZYGAL_ASYNC" = 'remote' ] && ! $ZYGAL_ENABLE_VCS_REMOTE; then
    echo >&2 "Remote features disabled (ZYGAL_ENABLE_VCS_REMOTE) but \
ZYGAL_ASYNC set to to \"remote\""
    false
else
    true
fi
