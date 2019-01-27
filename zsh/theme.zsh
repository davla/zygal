#!/usr/bin/env zsh

ZYGAL_THEME_ROOT=${${(%):-%x}:h:h:P}

source "$ZYGAL_THEME_ROOT/lib/vcs.sh"

ZYGAL_ASYNC="${ZYGAL_ASYNC-remote}"

if [ "$ZYGAL_ASYNC" != 'none' ]; then
    source "$ZYGAL_THEME_ROOT/zsh/async.zsh"
    zygal_async_init
fi

# Escape sequence to reset all the prompt styles
ZYGAL_RESET='%f%k'

PROMPT_SUBST=true

# If this is an xterm set the title to user@host:dir
case "$TERM" in
    xterm*|rxvt*)
        alias zygal_xterm_title="print -Pn '\033]2;%n@%M %2(~.*/%1~.%~)\007'"
        ;;
    *)
        alias zygal_xterm_title=true
        ;;
esac

zygal_theme() {
    zygal_xterm_title

    local COLORSCHEME="${1:-${ZYGAL_COLORSCHEME:-orange}}"

    [ ! -f "$COLORSCHEME" ] \
        && COLORSCHEME="$ZYGAL_THEME_ROOT/colorschemes/$COLORSCHEME.sh"

    source "$COLORSCHEME"

    typeset -g ZYGAL_PRE_VCS="%F{$TEXT_COLOR}%K{$USER_HOST_BG} %n@%M \
%K{$CWD_BG} %2(~.*/%1~.%~) $ZYGAL_RESET"
    typeset -g ZYGAL_POST_VCS=$'\n'"%F{$TEXT_COLOR}%K{$USER_HOST_BG} \
└─%# $ZYGAL_RESET "
    typeset -g ZYGAL_VCS_FORMAT="%%F{$TEXT_COLOR}%%K{$VCS_BG} [%s]%s \
${ZYGAL_RESET//\%/%%}"

    $ZYGAL_VCS_REMOTE \
        && ZYGAL_VCS_REMOTE_COUNT=$(( (ZYGAL_VCS_REMOTE_COUNT + 1) \
            % ZYGAL_VCS_REMOTE_SYNC_TRIGGER ))

    case "$ZYGAL_ASYNC" in
        'all')
            PROMPT="${ZYGAL_PRE_VCS}${ZYGAL_POST_VCS}"
            zygal_async
            ;;

        'remote')
            local ZYGAL_VCS="$(zygal_vcs_info "$ZYGAL_VCS_FORMAT")"
            PROMPT="${ZYGAL_PRE_VCS}${ZYGAL_VCS}${ZYGAL_POST_VCS}"
            zygal_async
            ;;

        'none')
            [ "$ZYGAL_VCS_REMOTE_COUNT" -eq 0 ] \
                && local ZYGAL_VCS="$(zygal_vcs_info_remote \
                    "$ZYGAL_VCS_FORMAT")" \
                || local ZYGAL_VCS="$(zygal_vcs_info "$ZYGAL_VCS_FORMAT")"
            PROMPT="${ZYGAL_PRE_VCS}${ZYGAL_VCS}${ZYGAL_POST_VCS}"
            ;;
    esac
}

add-zsh-hook precmd zygal_theme
