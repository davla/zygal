#!/usr/bin/env zsh

# Escape sequence to reset all the prompt styles
RESET='%f%k'

THEME_ROOT="$(readlink -f "${${(%):-%x}:h}/..")"
source "$THEME_ROOT/lib/vcs.sh"

export PROMPT_SUBST=1

zygal-theme() {
    local COLORSCHEME="${1:-orange}"

    source "$THEME_ROOT/colorschemes/$COLORSCHEME.sh"

    typeset -g ZYGAL_PRE_VCS="%F{$TEXT_COLOR}%K{$USER_HOST_BG} %n@%M %K{$CWD_BG} \
%2(~.*/%1~.%~) $RESET"
    # readonly ZYGAL_PRE_VCS
    typeset -g ZYGAL_POST_VCS=$'\n'"%F{$TEXT_COLOR}%K{$USER_HOST_BG} └─%# $RESET "
    # readonly ZYGAL_POST_VCS
    local ZYGAL_VCS="%%F{$TEXT_COLOR}%%K{$VCS_BG} [%s]%s ${RESET//\%/%%}"

    source "$THEME_ROOT/zsh/async.zsh"

    PROMPT="${ZYGAL_PRE_VCS}${ZYGAL_POST_VCS}"
}

add-zsh-hook precmd zygal-theme
