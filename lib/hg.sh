#!/bin/sh

zygal_hg_prompt_info() {
    local STATUS
    STATUS="$(hg status -marduT '{status}' 2> /dev/null)"
    [ $? -ne 0 ] && return

    grep '\(M\|A\|R\)' <<<"$STATUS" &> /dev/null && local DIRTY='+'
    grep '?' <<<"$STATUS" &> /dev/null && local UNTRACKED='%%'
    grep '!' <<<"$STATUS" &> /dev/null && local MISSING='!'
    [ -n "$(hg shelve -l 2> /dev/null)" ] && local SHELVES='$'

    hg identify -T "{separate(' ',
            if(activebookmark,activebookmark,branch),
            '${DIRTY}${SHELVES}${UNTRACKED}${MISSING}')}" \
        | xargs -i printf "$1" '{}' ' hg'
}
