#!/usr/bin/env zsh

# Escape sequence to reset all the prompt styles
RESET='%f%k'

THEME_ROOT="$(print -P %x | xargs dirname | xargs -i readlink -f '{}/..')"
source "$THEME_ROOT/lib/vcs.sh"

export PROMPT_SUBST=1

zygal-theme() {
    local COLORSCHEME="${1:-orange}"

    source "$THEME_ROOT/colorschemes/$COLORSCHEME.sh"

    local ZYGAL_PRE_VCS="%F{$TEXT_COLOR}%K{$USER_HOST_BG} %n@%M %K{$CWD_BG} \
%2(~.*/%1~.%~) $RESET"
    local ZYGAL_POST_VCS=$'\n'"%F{$TEXT_COLOR}%K{$USER_HOST_BG} └─%# $RESET "
    local ZYGAL_VCS="$(vcs_info "%%F{$TEXT_COLOR}%%K{$VCS_BG} [%s] ${RESET//\%/%%}")"

    PROMPT="${ZYGAL_PRE_VCS}${ZYGAL_VCS}${ZYGAL_POST_VCS}"
}

add-zsh-hook precmd zygal-theme
