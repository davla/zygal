#!/bin/sh

zygal_hg_prompt_info() {
    local STATUS
    STATUS="$(hg status -marduT '{status}' 2> /dev/null)"
    [ $? -ne 0 ] && return

    grep '\(M\|A\|R\)' <<<"$STATUS" &> /dev/null && local DIRTY='+'
    grep '?' <<<"$STATUS" &> /dev/null && local UNTRACKED='%%'
    grep '!' <<<"$STATUS" &> /dev/null && local MISSING='!'
    [ -n "$(hg shelve -l 2> /dev/null)" ] && local SHELVES='$'

    local REMOTE=''
    hg incoming -q .hg/changesets.hg &> /dev/null && REMOTE='<'
    [ -n "$(hg log -r 'draft()' -l 1 2> /dev/null)" ] && REMOTE="${REMOTE}>"
    REMOTE="${REMOTE:-=}"    

    hg identify -T "{separate(' ',
            if(activebookmark,activebookmark,branch),
            '${DIRTY}${SHELVES}${UNTRACKED}${MISSING}${REMOTE}')}" \
        | xargs -i printf "$1" '{}' ' hg'
}

zygal_hg_sync_remote() {
    hg incoming --bundle .hg/changesets.hg &> /dev/null
}
