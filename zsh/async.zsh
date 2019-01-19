#!/usr/bin/env zsh

PARENT_DIR="${${(%):-%x}:h:P}"

source "$PARENT_DIR/../deps/zsh-async/async.zsh"
source "$PARENT_DIR/../lib/git.sh"

append_git() {
    PROMPT="${ZYGAL_PRE_VCS}${3}${ZYGAL_POST_VCS}"
    zle reset-prompt
}

async_init

async_start_worker git_base
async_register_callback git_base append_git
async_job git_base git_info $ZYGAL_VCS

async_start_worker git_remote
async_register_callback git_remote append_git
async_job git_remote git_remote $ZYGAL_VCS
