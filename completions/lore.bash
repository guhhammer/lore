# bash completion for lore. lore itself says what can come next
# (`lore --complete WORDS`); with nothing to offer, file names.
# Installed by the packages; or add to ~/.bashrc:
#   source <(lore --completions bash)

_lore() {
    local cur=${COMP_WORDS[COMP_CWORD]}
    # one candidate a line, taken as it is: `[` is a command, not a pattern
    mapfile -t COMPREPLY < <(lore --complete "${COMP_WORDS[@]:1:COMP_CWORD}" 2>/dev/null | cut -f1)
    if (( ${#COMPREPLY[@]} == 0 )); then
        compopt -o filenames # slashes after folders, spaces escaped
        mapfile -t COMPREPLY < <(compgen -f -- "$cur")
    fi
}
complete -F _lore lore
