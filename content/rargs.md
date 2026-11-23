# rargs

Execute a command for each line of `stdin`. Like `xargs`, but with pattern matching support.

## Usage
- `command | rargs command {0}` : execute a command for every line of input, just like `xargs` (`{0}` indicates where to substitute in the text)
- `command | rargs -e command {0}` : simulate which commands would be run without executing them
- `command | rargs -p '(.*).bak' mv {0} {1}` : remove the `.bak` extension from every file in a list
- `command | rargs -w max_procs` : execute commands in parallel
- `command | rargs -0 command {0}` : consider each line of input to be separated by a NUL character (`\0`) instead of a newline (`\n`)

## Categories
text, macos

## Links
- More information: https://github.com/lotabout/rargs
- tldr page: https://tldr.inbrowser.app/pages/osx/rargs
