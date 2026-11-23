# whence

A Zsh builtin to indicate how a command would be interpreted.

## Usage
- `whence "command"` : interpret `command`, with expansion if defined as an `alias` (similar to the `command -v` builtin)
- `whence -v "command"` : display type of `command`, with location if defined as a function or binary (equivalent to the `type` and `command -V` builtins)
- `whence -c "command"` : display type of `command`, with content of shell functions instead of location (equivalent to `which` builtin)
- `whence -ca "command"` : display type of `command`, with content of shell functions and all occurrences on command path (equivalent to the `where` builtin)
- `whence -p "command"` : search only the `$PATH` for `command`, ignoring builtins, aliases, or shell functions (equivalent to the `where` command)

## Categories
shell, desktop

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/whence
