#!/usr/bin/env zsh

ZYGAL_THEME_ROOT="${${(%):-%x}:h:h:P}"

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

echo -n "readonly ZYGAL_PRE_VCS='%F{$TEXT_COLOR}%K{$USER_HOST_BG} %n@%M "
echo "%K{$CWD_BG} %2(~.*/%1~.%~) $ZYGAL_RESET'"
echo -n "readonly ZYGAL_VCS_FORMAT='%%F{$TEXT_COLOR}%%K{$VCS_BG} [%s]%s "
echo "${ZYGAL_RESET//\%/%%}'"
echo -n "readonly ZYGAL_POST_VCS=$'\\\\n''%F{$TEXT_COLOR}%K{$USER_HOST_BG} "
echo "└─%# $ZYGAL_RESET '"

echo 'PROMPT_SUBST=true'

source "$ZYGAL_THEME_ROOT/lib/git.sh"
typeset | grep -P '^GIT_PS1'
grep -P 'source.+git-core' "$ZYGAL_THEME_ROOT/lib/git.sh"
type -f zygal_git_prompt_info
type -f zygal_git_sync_remote

source "$ZYGAL_THEME_ROOT/lib/hg.sh"
type -f zygal_hg_prompt_info
# type -f zygal_hg_sync_remote

source "$ZYGAL_THEME_ROOT/lib/vcs.sh"
type -f zygal_vcs_info
type -f zygal_vcs_info_remote

ZYGAL_ASYNC="${ZYGAL_ASYNC-remote}"

if [ "$ZYGAL_ASYNC" != 'none' ]; then
    echo "source \"$ZYGAL_THEME_ROOT/deps/zsh-async/async.zsh\""

    source "$ZYGAL_THEME_ROOT/zsh/async.zsh"
    type -f zygal_append_vcs
    type -f zygal_append_vcs_and_stop
    echo 'async_init'

    [ "$ZYGAL_ASYNC" = 'all' ] && {
        echo 'async_start_worker zygal_worker_vcs_base'
        echo 'async_register_callback zygal_worker_vcs_base zygal_append_vcs'
    }

    echo 'zygal_async() {'
    PWD_CMD='cd $PWD'

    [ "$ZYGAL_ASYNC" = 'all' ] && {
        echo "\tasync_worker_eval zygal_worker_vcs_base \"$PWD_CMD\""
        echo -n '\tasync_job zygal_worker_vcs_base zygal_vcs_info '
        echo '"$ZYGAL_VCS_FORMAT"'
    }

    $ZYGAL_ENABLE_VCS_REMOTE && {
        echo '\t[ "$ZYGAL_VCS_REMOTE_COUNT" -eq 0 ] && {'
        echo "\t\ttypeset -g ZYGAL_WORKER_NAME='zygal_worker_vcs_remote'"

        echo '\t\tasync_start_worker "$ZYGAL_WORKER_NAME"'
        echo -n '\t\tasync_register_callback "$ZYGAL_WORKER_NAME" '
        echo 'zygal_append_vcs_and_stop'
        echo "\t\tasync_worker_eval \"\$ZYGAL_WORKER_NAME\" \"$PWD_CMD\""
        echo '\t\tasync_job "$ZYGAL_WORKER_NAME" zygal_vcs_info_remote \'
        echo '\t\t\t"$ZYGAL_VCS_FORMAT"'
        echo '\t}'
    }

    echo '}'
fi

alias add-zsh-hook='true'
source "$ZYGAL_THEME_ROOT/zsh/theme.zsh" > /dev/null
type -f zygal_xterm_title

echo 'zygal-theme() {'
    $ZYGAL_VCS_REMOTE && {
        echo -n '\tZYGAL_VCS_REMOTE_COUNT=$(( (ZYGAL_VCS_REMOTE_COUNT + 1) % '
        echo "$ZYGAL_VCS_REMOTE_SYNC_TRIGGER ))"
    }

    case "$ZYGAL_ASYNC" in
        'all')
            echo 'PROMPT="${ZYGAL_PRE_VCS}${ZYGAL_POST_VCS}"'
            echo 'zygal_async'
            ;;

        'remote')
            echo '\tlocal ZYGAL_VCS="$(zygal_vcs_info "$ZYGAL_VCS_FORMAT")"'
            echo '\tPROMPT="${ZYGAL_PRE_VCS}${ZYGAL_VCS}${ZYGAL_POST_VCS}"'
            echo '\tzygal_async'
            ;;

        'none')
            echo '\tlocal ZYGAL_VCS="$(zygal_vcs_info_remote "$ZYGAL_VCS")"'
            echo '\tPROMPT="${ZYGAL_PRE_VCS}${ZYGAL_VCS}${ZYGAL_POST_VCS}"'
            ;;
    esac
echo '}'

echo 'add-zsh-hook chpwd zygal_xterm_title'
echo 'add-zsh-hook precmd zygal-theme'
