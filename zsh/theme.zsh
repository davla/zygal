#!/usr/bin/env zsh

# Escape sequence to reset all the prompt styles
ZYGAL_RESET='%f%k'

ZYGAL_THEME_ROOT=${${(%):-%x}:h:h}
source "$ZYGAL_THEME_ROOT/lib/vcs.sh"

PROMPT_SUBST=true
ZYGAL_ASYNC="${ZYGAL_ASYNC-true}"

if $ZYGAL_ASYNC; then
    source "$ZYGAL_THEME_ROOT/zsh/async.zsh"
    zygal_async_init
fi

zygal_theme() {
    local COLORSCHEME="${1:-orange}"

    source "$ZYGAL_THEME_ROOT/colorschemes/$COLORSCHEME.sh"

    typeset -g ZYGAL_PRE_VCS="%F{$TEXT_COLOR}%K{$USER_HOST_BG} %n@%M \
%K{$CWD_BG} %2(~.*/%1~.%~) $ZYGAL_RESET"
    typeset -g ZYGAL_POST_VCS=$'\n'"%F{$TEXT_COLOR}%K{$USER_HOST_BG} \
└─%# $ZYGAL_RESET "
    typeset -g ZYGAL_VCS="%%F{$TEXT_COLOR}%%K{$VCS_BG} [%s]%s \
${ZYGAL_RESET//\%/%%}"

    if $ZYGAL_ASYNC; then
        PROMPT="${ZYGAL_PRE_VCS}${ZYGAL_POST_VCS}"
        zygal_async
    else
        ZYGAL_VCS="$(zygal_vcs_info "$ZYGAL_VCS")"
        PROMPT="${ZYGAL_PRE_VCS}${ZYGAL_VCS}${ZYGAL_POST_VCS}"
    fi
}

add-zsh-hook precmd zygal_theme
