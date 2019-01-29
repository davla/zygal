#!/usr/bin/env zsh

ZYGAL_THEME_ROOT="${${(%):-%x}:h:h:P}"
source "$ZYGAL_THEME_ROOT/lib/config.sh"

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

echo -n "readonly ZYGAL_PRE_VCS='%F{$ZYGAL_TEXT_COLOR}%K{$ZYGAL_USER_HOST_BG} "
echo "%n@%M %K{$ZYGAL_CWD_BG} %2(~.*/%1~.%~) $ZYGAL_RESET'"
echo -n "readonly ZYGAL_VCS_FORMAT='%%F{$ZYGAL_TEXT_COLOR}%%K{$ZYGAL_VCS_BG} "
echo "[%s]%s ${ZYGAL_RESET//\%/%%}'"
echo -n "readonly ZYGAL_POST_VCS=$'\\\\n''%F{$ZYGAL_TEXT_COLOR}"
echo "%K{$ZYGAL_USER_HOST_BG} └─%# $ZYGAL_RESET '"

echo 'PROMPT_SUBST=true'

source "$ZYGAL_THEME_ROOT/lib/git.sh"
source "$ZYGAL_THEME_ROOT/lib/hg.sh"
source "$ZYGAL_THEME_ROOT/lib/vcs.sh"

typeset | grep -P '^GIT_PS1'
echo "type __git_ps1 &> /dev/null || source $ZYGAL_GIT_PS1_PATH"

type -f zygal_git_prompt_info
type -f zygal_hg_prompt_info
type -f zygal_vcs_info

$ZYGAL_ENABLE_VCS_REMOTE && {
    type -f zygal_git_sync_remote
    # type -f zygal_hg_sync_remote
    type -f zygal_vcs_info_remote
}

[ "$ZYGAL_ASYNC" != 'none' ] && {
    echo "source $ZYGAL_ZSH_ASYNC_PATH"

    source "$ZYGAL_THEME_ROOT/zsh/async.zsh"
    type -f zygal_append_vcs
    $ZYGAL_ENABLE_VCS_REMOTE && type -f zygal_append_vcs_and_stop
    echo 'async_init'

    [ "$ZYGAL_ASYNC" = 'all' ] && {
        echo 'async_start_worker zygal_worker_vcs_base'
        echo 'async_register_callback zygal_worker_vcs_base zygal_append_vcs'
    }

    echo 'zygal_async() {'

    [ "$ZYGAL_ASYNC" = 'all' ] && {
        echo '\tasync_worker_eval zygal_worker_vcs_base "cd $PWD"'
        echo -n '\tasync_job zygal_worker_vcs_base zygal_vcs_info '
        echo '"$ZYGAL_VCS_FORMAT"'
    }

    $ZYGAL_ENABLE_VCS_REMOTE && {
        echo '\t[ "$ZYGAL_VCS_REMOTE_COUNT" -eq 0 ] && {'
        echo "\t\ttypeset -g ZYGAL_WORKER_NAME='zygal_worker_vcs_remote'"

        echo '\t\tasync_start_worker "$ZYGAL_WORKER_NAME"'
        echo -n '\t\tasync_register_callback "$ZYGAL_WORKER_NAME" '
        echo 'zygal_append_vcs_and_stop'
        echo '\t\tasync_job "$ZYGAL_WORKER_NAME" zygal_vcs_info_remote \'
        echo '\t\t\t"$ZYGAL_VCS_FORMAT"'
        echo '\t}'
    }

    echo '}'
}

echo 'case "$TERM" in'
echo '\txterm*|rxvt*)'
echo -n '\t\talias zygal_xterm_title="print -Pn '
echo "'\\\\033]2;%n@%M %2(~.*/%1~.%~)\\\\007'\""
echo '\t\t;;'
echo '\t*)'
echo '\t\talias zygal_xterm_title=true'
echo '\t\t;;'
echo 'esac'

echo 'zygal-theme() {'

echo '\tzygal_xterm_title'
$ZYGAL_ENABLE_VCS_REMOTE && {
    echo -n '\tZYGAL_VCS_REMOTE_COUNT=$(( (ZYGAL_VCS_REMOTE_COUNT + 1) % '
    echo "$ZYGAL_VCS_REMOTE_SYNC_TRIGGER ))"
}

case "$ZYGAL_ASYNC" in
    'all')
        echo '\tPROMPT="${ZYGAL_PRE_VCS}${ZYGAL_POST_VCS}"'
        echo '\tzygal_async'
        ;;

    'remote')
        echo '\tlocal ZYGAL_VCS="$(zygal_vcs_info "$ZYGAL_VCS_FORMAT")"'
        echo '\tPROMPT="${ZYGAL_PRE_VCS}${ZYGAL_VCS}${ZYGAL_POST_VCS}"'
        echo '\tzygal_async'
        ;;

    'none')
        if $ZYGAL_ENABLE_VCS_REMOTE; then
            echo '\t[ "$ZYGAL_VCS_REMOTE_COUNT" -eq 0 ] \'
            echo -n '\t\t&& local ZYGAL_VCS="$(zygal_vcs_info_remote '
            echo '"$ZYGAL_VCS_FORMAT")" \'
            echo -n '\t\t|| local ZYGAL_VCS="$(zygal_vcs_info '
            echo '"$ZYGAL_VCS_FORMAT")"'
        else
            echo '\tlocal ZYGAL_VCS="$(zygal_vcs_info "$ZYGAL_VCS_FORMAT")"'
        fi
        echo '\tPROMPT="${ZYGAL_PRE_VCS}${ZYGAL_VCS}${ZYGAL_POST_VCS}"'
        ;;
esac
echo '}'

echo 'add-zsh-hook precmd zygal-theme'
