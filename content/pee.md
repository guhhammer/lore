# pee

Tee `stdin` to pipes.

## Usage
- `pee command1 command2 ...` : run each command, providing each one with a distinct copy of `stdin`
- `pee cat command1 command2 ...` : write a copy of `stdin` to `stdout` (like `tee`)
- `pee --no-ignore-sigpipe --no-ignore-write-errors command1 command2 ...` : immediately terminate upon `SIGPIPE`s and write errors

## See also
- tee

## Links
- More information: https://manned.org/pee
- tldr page: https://tldr.inbrowser.app/pages/common/pee
