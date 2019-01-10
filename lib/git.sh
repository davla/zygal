#!/usr/bin/env sh

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

# The separator between the branch name and the various indicators.
# shellcheck disable=SC2034
GIT_PS1_STATESEPARATOR=${GIT_PS1_STATESEPARATOR-' '}

type __git_ps1 1> /dev/null 2>&1 || . /usr/lib/git-core/git-sh-prompt

git_info() {
    __git_ps1 "${1-[%s]}"
}

git_remote() {
    GIT_PS1_SHOWUPSTREAM=${GIT_PS1_SHOWUPSTREAM-'auto'}
    __git_ps1 "${1-[%s]}"
    unset GIT_PS1_SHOWUPSTREAM
}
