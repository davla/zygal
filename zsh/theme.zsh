#!/usr/bin/env zsh

# Escape sequence to reset all the prompt styles
ZYGAL_RESET='%f%k'

ZYGAL_THEME_ROOT=${${(%):-%x}:h:h:P}
source "$ZYGAL_THEME_ROOT/lib/vcs.sh"

PROMPT_SUBST=true
ZYGAL_ASYNC="${ZYGAL_ASYNC-remote}"

if [ "$ZYGAL_ASYNC" != 'none' ]; then
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

    # If this is an xterm set the title to user@host:dir
    case "$TERM" in
        xterm*|rxvt*)
            print -Pn '\033]2;%n@%M %2(~.*/%1~.%~)\007'
            ;;
        *)
            ;;
    esac

    $ZYGAL_VCS_REMOTE \
        && ZYGAL_VCS_REMOTE_COUNT=$(( (ZYGAL_VCS_REMOTE_COUNT + 1) \
            % ZYGAL_VCS_REMOTE_SYNC_TRIGGER ))

    case "$ZYGAL_ASYNC" in
        'all')
            PROMPT="${ZYGAL_PRE_VCS}${ZYGAL_POST_VCS}"
            zygal_async
            ;;

        'remote')
            ZYGAL_VCS="$(zygal_vcs_info "$ZYGAL_VCS")"
            PROMPT="${ZYGAL_PRE_VCS}${ZYGAL_VCS}${ZYGAL_POST_VCS}"
            zygal_async
            ;;

        'none')
            ZYGAL_VCS="$(zygal_vcs_info_remote "$ZYGAL_VCS")"
            PROMPT="${ZYGAL_PRE_VCS}${ZYGAL_VCS}${ZYGAL_POST_VCS}"
            ;;
    esac
}

add-zsh-hook precmd zygal_theme
