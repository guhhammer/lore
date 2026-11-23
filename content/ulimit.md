# ulimit

Get and set resource limits for user processes. This is a shell builtin hence not shell-agnostic.

## Usage
- `ulimit -a` : get the properties of all the user limits
- `ulimit -H -n` : get hard limit for the number of simultaneously opened files
- `ulimit -S -n` : get soft limit for the number of simultaneously opened files
- `ulimit -u 30` : set max per-user process limit
- `ulimit -c|f|s|... unlimited` : remove limits for a specific resource
- `help ulimit` : display help (Bash only)

## Categories
shell, users, processes

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-ulimit
- tldr page: https://tldr.inbrowser.app/pages/common/ulimit
