#!/bin/sh

ZYGAL_HG_BUNDLE="${ZYGAL_HG_BUNDLE:-.hg/changesets.hg}"
ZYGAL_HG_DIRTY="${ZYGAL_HG_DIRTY-true}"
ZYGAL_HG_MISSING="${ZYGAL_HG_MISSING-true}"
ZYGAL_HG_REMOTE="${ZYGAL_HG_REMOTE-true}"
ZYGAL_HG_SEPARATOR="${ZYGAL_HG_SEPARATOR-' '}"
ZYGAL_HG_SHELVE="${ZYGAL_HG_SHELVE-true}"
ZYGAL_HG_UNTRACKED="${ZYGAL_HG_UNTRACKED-true}"

zygal_hg_prompt_info() {
    ZYGAL_HG_PROMPT_STATUS="$(hg status -marduT '{status}' 2> /dev/null)"
    [ $? -ne 0 ] && return

    $ZYGAL_HG_DIRTY \
        && echo "$ZYGAL_HG_PROMPT_STATUS" | grep '\(M\|A\|R\)' > /dev/null \
            2>&1 \
        && ZYGAL_HG_PROMPT_DIRTY='+'
    $ZYGAL_HG_UNTRACKED \
        && echo "$ZYGAL_HG_PROMPT_STATUS" | grep '?' > /dev/null 2>&1 \
        && ZYGAL_HG_PROMPT_UNTRACKED='%%'
    $ZYGAL_HG_MISSING \
        && echo "$ZYGAL_HG_PROMPT_STATUS" | grep '!' > /dev/null 2>&1 \
        && ZYGAL_HG_PROMPT_MISSING='!'
    $ZYGAL_HG_SHELVE \
        && [ -n "$(hg shelve -l 2> /dev/null)" ] \
        && ZYGAL_HG_PROMPT_SHELVES='$'

    $ZYGAL_HG_REMOTE && {
        ZYGAL_HG_PROMPT_REMOTE=''
        hg incoming -q .hg/changesets.hg > /dev/null 2>&1 \
            && ZYGAL_HG_PROMPT_REMOTE='<'
        [ -n "$(hg log -r 'draft()' -l 1)" ] \
            && ZYGAL_HG_PROMPT_REMOTE="${ZYGAL_HG_PROMPT_REMOTE}>"
        ZYGAL_HG_PROMPT_REMOTE="${ZYGAL_HG_PROMPT_REMOTE:-=}"
    }

    hg identify -T "{separate($ZYGAL_HG_SEPARATOR,
            if(activebookmark,activebookmark,branch),
'${ZYGAL_HG_PROMPT_DIRTY}\
${ZYGAL_HG_PROMPT_SHELVES}\
${ZYGAL_HG_PROMPT_UNTRACKED}\
${ZYGAL_HG_PROMPT_MISSING}\
${ZYGAL_HG_PROMPT_REMOTE}')}" \
        | xargs -I '{}' printf "$1" '{}' ' hg'

    unset ZYGAL_HG_PROMPT_STATUS ZYGAL_HG_PROMPT_DIRTY
    unset ZYGAL_HG_PROMPT_UNTRACKED ZYGAL_HG_PROMPT_MISSING
    unset ZYGAL_HG_PROMPT_SHELVES ZYGAL_HG_PROMPT_REMOTE
}

zygal_hg_sync_remote() {
    hg incoming --bundle "$ZYGAL_HG_BUNDLE" > /dev/null 2>&1
}
