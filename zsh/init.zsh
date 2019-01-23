#!/usr/bin/env zsh

ZYGAL_THEME_ROOT="${${(%):-%x}:h:h:P}"

###############################################################################
#
#                                   Colors
#
###############################################################################

[ -f "$ZYGAL_COLORSCHEME" ] \
    && COLORSCHEME="$ZYGAL_COLORSCHEME" \
    || COLORSCHEME="$ZYGAL_THEME_ROOT/colorschemes/$ZYGAL_COLORSCHEME.sh"

source "$COLORSCHEME"

ZYGAL_RESET='%f%k'

echo "readonly ZYGAL_PRE_VCS='%F{$TEXT_COLOR}%K{$USER_HOST_BG} %n@%M %K{$CWD_BG} %2(~.*/%1~.%~) $ZYGAL_RESET'"
echo "readonly ZYGAL_VCS_FORMAT='%%F{$TEXT_COLOR}%%K{$VCS_BG} [%s]%s ${ZYGAL_RESET//\%/%%}'"
echo "readonly ZYGAL_POST_VCS=$'\\\\n''%F{$TEXT_COLOR}%K{$USER_HOST_BG} └─%# $ZYGAL_RESET '"

echo "case \"\$TERM\" in
    xterm*|rxvt*)
        print -Pn '\\\\033]2;%n@%M %2(~.*/%1~.%~)\\\\007'
        ;;
    *)
        ;;
esac"
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
    echo 'async_init'

    [ "$ZYGAL_ASYNC" = 'all' ] && {
        echo 'async_start_worker zygal_vcs_base'
        echo 'async_register_callback zygal_vcs_base zygal_append_vcs'
    }

    $ZYGAL_ENABLE_VCS_REMOTE && {
        echo 'async_start_worker zygal_vcs_remote'
        echo 'async_register_callback zygal_vcs_remote zygal_append_vcs'
    }

    echo 'zygal_async() {'
    PWD_CMD='"cd $PWD"'

    [ "$ZYGAL_ASYNC" = 'all' ] && {
        echo "\tasync_worker_eval zygal_vcs_base \"$PWD_CMD\""
        echo '\tasync_job zygal_vcs_base zygal_vcs_info "$ZYGAL_VCS_FORMAT"'
    }

    $ZYGAL_ENABLE_VCS_REMOTE && {
        echo "\tasync_worker_eval zygal_vcs_remote \"$PWD_CMD\""
        echo '\tasync_job zygal_vcs_remote zygal_vcs_info_remote "$ZYGAL_VCS"'
    }

    echo '}'
fi

echo 'zygal-theme() {'
    $ZYGAL_VCS_REMOTE \
        && echo "\tZYGAL_VCS_REMOTE_COUNT=\$(( (ZYGAL_VCS_REMOTE_COUNT + 1) % $ZYGAL_VCS_REMOTE_SYNC_TRIGGER ))"

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
echo 'add-zsh-hook precmd zygal-theme'
