#!/usr/bin/env zsh

[ -n "$ZYGAL_THEME_ROOT" ] && ROOT_DEFINED=true || ROOT_DEFINED=false
$ROOT_DEFINED || {
    THIS_FILE="$(readlink -f "${(%):-%x}")"
    ZYGAL_THEME_ROOT="${THIS_FILE:h:h}"
    unset THIS_FILE
}

fpath=("$ZYGAL_THEME_ROOT/zsh" $fpath)
autoload -Uz zygal-static

$ROOT_DEFINED || unset ZYGAL_THEME_ROOT
unset ROOT_DEFINED
