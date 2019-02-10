#!/bin/sh

# shellcheck disable=SC2034

# Whether to show modifications of tracked files (*/+)
GIT_PS1_SHOWDIRTYSTATE=${GIT_PS1_SHOWDIRTYSTATE-'1'}

# Whether to show the presence of stashed elements ($)
GIT_PS1_SHOWSTASHSTATE=${GIT_PS1_SHOWSTASHSTATE-'1'}

# Whether to show the presence of untracked elements (%)
GIT_PS1_SHOWUNTRACKEDFILES=${GIT_PS1_SHOWUNTRACKEDFILES-'1'}

# Showing minimal information about the upstream (<, >, = or <>)
GIT_PS1_SHOWUPSTREAM=${GIT_PS1_SHOWUPSTREAM-'auto'}

# The separator between the branch name and the various indicators.
GIT_PS1_STATESEPARATOR=${GIT_PS1_STATESEPARATOR-' '}

type __git_ps1 > /dev/null 2>&1  \
    || . "${ZYGAL_GIT_PROMPT_PATH:-/usr/lib/git-core/git-sh-prompt}"

zygal_git_prompt_info() {
    git status > /dev/null 2>&1 || return

    ZYGAL_GIT_PROMPT_INFO="$(__git_ps1 '%s')"
    ZYGAL_GIT_PROMPT_BRANCH="$(git symbolic-ref --short HEAD 2> /dev/null)"

    [ -n "$ZYGAL_GIT_PROMPT_BRANCH" ] \
        && [ "$ZYGAL_GIT_PROMPT_INFO" != "$ZYGAL_GIT_PROMPT_BRANCH" ] \
        && ZYGAL_GIT_PROMPT_INFO="$(echo "$ZYGAL_GIT_PROMPT_INFO" \
            | sed 's/|/\|/g' \
            | sed -E "s|$ZYGAL_GIT_PROMPT_BRANCH\
($GIT_PS1_STATESEPARATOR)*|${ZYGAL_GIT_PROMPT_BRANCH}\
${GIT_PS1_STATESEPARATOR}|")"

    printf -- "$1" "$ZYGAL_GIT_PROMPT_INFO"

    unset ZYGAL_GIT_PROMPT_INFO ZYGAL_GIT_PROMPT_BRANCH
}

zygal_git_sync_remote() {
    git status > /dev/null 2>&1 && git fetch
}
