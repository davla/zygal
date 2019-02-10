#!/bin/sh

ZYGAL_HG_BUNDLE="${ZYGAL_HG_BUNDLE:-.hg/changesets.hg}"
ZYGAL_HG_DIRTY="${ZYGAL_HG_DIRTY-true}"
ZYGAL_HG_MISSING="${ZYGAL_HG_MISSING-true}"
ZYGAL_HG_REMOTE="${ZYGAL_HG_REMOTE-true}"
ZYGAL_HG_SEPARATOR="${ZYGAL_HG_SEPARATOR-' '}"
ZYGAL_HG_SHELVE="${ZYGAL_HG_SHELVE-true}"
ZYGAL_HG_UNTRACKED="${ZYGAL_HG_UNTRACKED-true}"

zygal_hg_prompt_info() {
    local STATUS
    STATUS="$(hg status -marduT '{status}' 2> /dev/null)"
    [ $? -ne 0 ] && return

    $ZYGAL_HG_DIRTY \
        && grep '\(M\|A\|R\)' <<<"$STATUS" &> /dev/null && local DIRTY='+'
    $ZYGAL_HG_UNTRACKED \
        && grep '?' <<<"$STATUS" &> /dev/null && local UNTRACKED='%%'

    $ZYGAL_HG_MISSING \
        && grep '!' <<<"$STATUS" &> /dev/null && local MISSING='!'
    $ZYGAL_HG_SHELVE \
        && [ -n "$(hg shelve -l 2> /dev/null)" ] && local SHELVES='$'

    $ZYGAL_HG_REMOTE && {
        local REMOTE=''
        hg incoming -q .hg/changesets.hg &> /dev/null && REMOTE='<'
        [ -n "$(hg log -r 'draft()' -l 1)" ] && REMOTE="${REMOTE}>"
        REMOTE="${REMOTE:-=}"
    }

    hg identify -T "{separate($ZYGAL_HG_SEPARATOR,
            if(activebookmark,activebookmark,branch),
            '${DIRTY}${SHELVES}${UNTRACKED}${MISSING}${REMOTE}')}" \
        | xargs -i printf "$1" '{}' ' hg'
}

zygal_hg_sync_remote() {
    hg incoming --bundle "$ZYGAL_HG_BUNDLE" &> /dev/null
}
