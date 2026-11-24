# fish completion for lore. lore itself says what can come next
# (`lore --complete WORDS`), with what each one is; with nothing to offer,
# file names. Installed by the packages; or:
#   lore --completions fish > ~/.config/fish/completions/lore.fish

function __lore_complete
    set -l found (lore --complete (commandline -opc)[2..-1] (commandline -ct) 2>/dev/null)
    if test (count $found) -gt 0
        printf '%s\n' $found
    else
        __fish_complete_path (commandline -ct)
    end
end

complete -c lore -f -a '(__lore_complete)'
