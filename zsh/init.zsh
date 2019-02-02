#!/usr/bin/env zsh

THIS_FILE="$(readlink -f "${(%):-%x}")"
ZYGAL_THEME_ROOT="${THIS_FILE:h:h}"
unset THIS_FILE

source "$ZYGAL_THEME_ROOT/lib/config.sh"
[ $? -eq 0 ] && {

###############################################################################
#
#                                   Colors
#
###############################################################################

    echo '#!/usr/bin/env zsh'

    [ -f "$ZYGAL_COLORSCHEME" ] \
        && COLORSCHEME="$ZYGAL_COLORSCHEME" \
        || COLORSCHEME="$ZYGAL_THEME_ROOT/colorschemes/$ZYGAL_COLORSCHEME.sh"

    source "$COLORSCHEME"

    ZYGAL_RESET='%f%k'
    ZYGAL_CWD_FORMAT='%3(~.*/%1~.%~)'

    cat <<GLOBSEOF
readonly ZYGAL_PRE_VCS='%F{$ZYGAL_TEXT_COLOR}%K{$ZYGAL_USER_HOST_BG} \
%n@%M %K{$ZYGAL_CWD_BG} $ZYGAL_CWD_FORMAT $ZYGAL_RESET'
readonly ZYGAL_VCS_FORMAT='%%F{$ZYGAL_TEXT_COLOR}%%K{$ZYGAL_VCS_BG} \
[%s]%s ${ZYGAL_RESET//\%/%%}'
readonly ZYGAL_POST_VCS=$'\\n''%F{$ZYGAL_TEXT_COLOR}%K{$ZYGAL_USER_HOST_BG} \
└─%# $ZYGAL_RESET '
PROMPT_SUBST=true
GLOBSEOF

    source "$ZYGAL_THEME_ROOT/lib/git.sh"
    source "$ZYGAL_THEME_ROOT/lib/hg.sh"
    source "$ZYGAL_THEME_ROOT/lib/vcs.sh"

    typeset | grep -P '^GIT_PS1'
    echo "type __git_ps1 &> /dev/null || source $ZYGAL_GIT_PROMPT_PATH"

    type -f zygal_git_prompt_info
    type -f zygal_hg_prompt_info
    type -f zygal_vcs_info

    $ZYGAL_ENABLE_VCS_REMOTE && {
        type -f zygal_git_sync_remote
        # type -f zygal_hg_sync_remote
        type -f zygal_vcs_info_remote
    }

    [ "$ZYGAL_ASYNC" != 'none' ] && {
        echo "source $ZYGAL_ZSH_ASYNC_PATH/async.zsh"

        source "$ZYGAL_THEME_ROOT/zsh/async.zsh"
        type -f zygal_append_vcs
        $ZYGAL_ENABLE_VCS_REMOTE && type -f zygal_append_vcs_and_stop
        echo 'async_init'

        [ "$ZYGAL_ASYNC" = 'all' ] && cat <<ASYNCEOF
async_start_worker zygal_worker_vcs_base
async_register_callback zygal_worker_vcs_base zygal_append_vcs
ASYNCEOF

        echo 'zygal_async() {'

        [ "$ZYGAL_ASYNC" = 'all' ] && cat <<ASYNCEOF
    async_worker_eval zygal_worker_vcs_base "cd \$PWD"
    async_job zygal_worker_vcs_base zygal_vcs_info "$ZYGAL_VCS_FORMAT"
ASYNCEOF

        $ZYGAL_ENABLE_VCS_REMOTE && cat <<REMOTEEOF
    [ "\$ZYGAL_VCS_REMOTE_COUNT" -eq 0 ] && {
        typeset -g ZYGAL_WORKER_NAME='zygal_worker_vcs_remote'
        async_start_worker "\$ZYGAL_WORKER_NAME"
        async_register_callback "\$ZYGAL_WORKER_NAME" zygal_append_vcs_and_stop
        async_job "\$ZYGAL_WORKER_NAME" zygal_vcs_info_remote \\
            "\$ZYGAL_VCS_FORMAT"
    }
REMOTEEOF
        echo '}'
    }

    cat <<THEMEEOF
case "\$TERM" in
    xterm*|rxvt*)
        alias zygal_xterm_title="print -Pn \
'\\\\033]2;%n@%M $ZYGAL_CWD_FORMAT\\\\007'"
        ;;

    *)
        alias zygal_xterm_title=true
        ;;
esac

zygal-theme() {
    zygal_xterm_title
THEMEEOF

    $ZYGAL_ENABLE_VCS_REMOTE && cat <<REMOTEEOF
    ZYGAL_VCS_REMOTE_COUNT=\$(( (ZYGAL_VCS_REMOTE_COUNT + 1) % \
$ZYGAL_VCS_REMOTE_SYNC_TRIGGER ))
REMOTEEOF

    case "$ZYGAL_ASYNC" in
        'all')
            cat <<ASYNCALLEOF
    PROMPT="\${ZYGAL_PRE_VCS}\${ZYGAL_POST_VCS}"'
    zygal_async
ASYNCALLEOF
            ;;

        'remote')
            cat <<ASYNCREMOTEEOF
    local ZYGAL_VCS="\$(zygal_vcs_info "\$ZYGAL_VCS_FORMAT")"
    PROMPT="\${ZYGAL_PRE_VCS}\${ZYGAL_VCS}\${ZYGAL_POST_VCS}"
    zygal_async
ASYNCREMOTEEOF
            ;;

        'none')
            if $ZYGAL_ENABLE_VCS_REMOTE; then
                cat <<ASYNCNONEEOF
    [ "\$ZYGAL_VCS_REMOTE_COUNT" -eq 0 ] \\
        && local ZYGAL_VCS="\$(zygal_vcs_info_remote "\$ZYGAL_VCS_FORMAT")" \
        || local ZYGAL_VCS="$(zygal_vcs_info "\$ZYGAL_VCS_FORMAT")"
ASYNCNONEEOF
            else
                cat <<ASYNCNONEEOF
    local ZYGAL_VCS="\$(zygal_vcs_info "$ZYGAL_VCS_FORMAT")"
ASYNCNONEEOF
            fi
            echo '\tPROMPT="${ZYGAL_PRE_VCS}${ZYGAL_VCS}${ZYGAL_POST_VCS}"'
            ;;
    esac
    echo '}'

    cat <<HOOKEOF
autoload -Uz add-zsh-hook
add-zsh-hook precmd zygal-theme
HOOKEOF
}
