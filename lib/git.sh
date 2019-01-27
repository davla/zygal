#!/bin/sh

# Git ps1 parameters:

# Whether to show modifications of tracked files (*/+)
# shellcheck disable=SC2034
GIT_PS1_SHOWDIRTYSTATE=${GIT_PS1_SHOWDIRTYSTATE-'1'}

# Whether to show the presence of stashed elements ($)
# shellcheck disable=SC2034
GIT_PS1_SHOWSTASHSTATE=${GIT_PS1_SHOWSTASHSTATE-'1'}

# Whether to show the presence of untracked elements (%)
# shellcheck disable=SC2034
GIT_PS1_SHOWUNTRACKEDFILES=${GIT_PS1_SHOWUNTRACKEDFILES-'1'}

# Showing minimal information about the upstream (<, >, = or <>)
GIT_PS1_SHOWUPSTREAM=${GIT_PS1_SHOWUPSTREAM-'auto'}

# The separator between the branch name and the various indicators.
# shellcheck disable=SC2034
GIT_PS1_STATESEPARATOR=${GIT_PS1_STATESEPARATOR-' '}

type __git_ps1 &> /dev/null \
    || source "${ZYGAL_GIT_PS1_PATH:-/usr/lib/git-core/git-sh-prompt}"

zygal_git_prompt_info() {
    git status &> /dev/null && {
        local FORMAT="$1"

        local GIT_INFO="$(__git_ps1 '%s')"
        local SEP="$GIT_PS1_STATESEPARATOR"
        local BRANCH="$(git symbolic-ref --short HEAD 2> /dev/null)"

        [ -n "$BRANCH" ] && [ "$GIT_INFO" != "$BRANCH" ] && \
            GIT_INFO="$(sed -E "s/$BRANCH($SEP)*/${BRANCH}${SEP}/" \
                <<<"$GIT_INFO")"

        printf -- "$FORMAT" "$GIT_INFO"
    }
}

zygal_git_sync_remote() {
    git status &> /dev/null && git fetch
}
