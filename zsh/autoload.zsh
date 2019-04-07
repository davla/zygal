#!/usr/bin/env zsh

THIS_FILE="$(readlink -f "${(%):-%x}")"
ZYGAL_THEME_ROOT="${THIS_FILE:h:h}"
unset THIS_FILE

fpath=("$ZYGAL_THEME_ROOT/zsh" $fpath)
autoload -Uz zygal-static

unset ZYGAL_THEME_ROOT
