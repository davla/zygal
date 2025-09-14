#!/usr/bin/env bash

zygal-theme() {
    PS1="$(${ZYGAL_PROMPT:-zygal-prompt})"
}

PROMPT_COMMAND=("${PROMPT_COMMAND[@]}" zygal-theme)
