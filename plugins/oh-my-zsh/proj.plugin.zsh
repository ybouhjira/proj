# proj — project management CLI plugin for oh-my-zsh
# https://github.com/ybouhjira/proj

# Shell wrapper for `proj cd` (child process can't change parent directory)
proj() {
    if [ "$1" = "cd" ]; then
        shift
        local dir
        dir=$(command proj cd "$@" 2>&1)
        if [ $? -eq 0 ] && [ -n "$dir" ] && [ -d "$dir" ]; then
            builtin cd "$dir"
        else
            echo "$dir" >&2
        fi
    else
        command proj "$@"
    fi
}

# Aliases
alias p='proj'
alias pl='proj ls'
alias pls='proj ls'
alias pcd='proj cd'
alias psync='proj sync'
alias pcheck='proj check'
alias pnew='proj new'
alias pinfo='proj info'
alias popen='proj open'

# Load completions from proj (clap_complete)
if command -v proj &>/dev/null; then
    eval "$(command proj completions zsh 2>/dev/null)"
fi
