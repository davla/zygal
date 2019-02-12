#!/usr/bin/env zsh

THIS_FILE="$(readlink -f "${(%):-%x}")"
ZYGAL_THEME_ROOT="${THIS_FILE:h:h}"
unset THIS_FILE

source "$ZYGAL_THEME_ROOT/lib/config.sh"
[ $? -eq 0 ] && {
    source "$ZYGAL_THEME_ROOT/lib/vcs.sh"

    [ "$ZYGAL_ASYNC" != 'none' ] && {
        source "$ZYGAL_THEME_ROOT/zsh/async.zsh"
        async_init
        typeset -g ZYGAL_ASYNC_RUNNING_COUNT=0
    }

    # Escape sequence to reset all the prompt styles
    ZYGAL_RESET='%f%k'
    ZYGAL_CWD_FORMAT='%3(~.*/%1~.%~)'

    PROMPT_SUBST=true

    zygal_theme() {
        # If this is an xterm set the title to user@host:dir
        case "$TERM" in
            xterm*|rxvt*)
            print -Pn "\\033]2;%n@%M $ZYGAL_CWD_FORMAT\\007"
            ;;
        esac

        local COLORSCHEME="${1:-$ZYGAL_COLORSCHEME}"

        [ ! -f "$COLORSCHEME" ] \
            && COLORSCHEME="$ZYGAL_THEME_ROOT/colorschemes/$COLORSCHEME.sh"

        source "$COLORSCHEME"

        typeset -g ZYGAL_PRE_VCS="%F{$ZYGAL_TEXT_COLOR}\
%K{$ZYGAL_USER_HOST_BG} %n@%M %K{$ZYGAL_CWD_BG} $ZYGAL_CWD_FORMAT $ZYGAL_RESET"
        typeset -g ZYGAL_POST_VCS=$'\n'"%F{$ZYGAL_TEXT_COLOR}\
%K{$ZYGAL_USER_HOST_BG} └─%# $ZYGAL_RESET "
        typeset -g ZYGAL_VCS_FORMAT="%%F{$ZYGAL_TEXT_COLOR}\
%%K{$ZYGAL_VCS_BG} [%s]%s ${ZYGAL_RESET//\%/%%}"

        $ZYGAL_ENABLE_VCS_REMOTE \
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
                if $ZYGAL_ENABLE_VCS_REMOTE \
                    && [ "$ZYGAL_VCS_REMOTE_COUNT" -eq 0 ]; then
                    local ZYGAL_VCS="$(zygal_vcs_info_remote \
                        "$ZYGAL_VCS_FORMAT")"
                else
                    local ZYGAL_VCS="$(zygal_vcs_info "$ZYGAL_VCS_FORMAT")"
                fi
                PROMPT="${ZYGAL_PRE_VCS}${ZYGAL_VCS}${ZYGAL_POST_VCS}"
                ;;
            esac
        }

        autoload -Uz add-zsh-hook
        add-zsh-hook precmd zygal_theme
}
