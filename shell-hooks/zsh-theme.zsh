#!/usr/bin/env zsh

zygal-theme() {
    PROMPT="$(${ZYGAL_PROMPT:-zygal-prompt})"
}

autoload -Uz add-zsh-hook
add-zsh-hook precmd zygal-theme
