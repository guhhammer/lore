# watchexec

Run arbitrary commands when files change.

## Usage
- `watchexec ls -la` : call `ls -la` when any file in the current directory changes
- `watchexec -e js,css,html make` : run `make` when any JavaScript, CSS, and HTML file in the current directory changes
- `watchexec -w lib -w src make` : run `make` when any file in the `lib` or `src` directory changes
- `watchexec -r --stop-signal SIGKILL my_server` : call/restart `my_server` when any file in the current directory changes, sending `SIGKILL` to stop the child process
- `watchexec -r --stop-signal SIGKILL --poll 10000 -e java command` : restart the execution of a command when any Java source file in the current directory changes, sending `SIGKILL` and only checking for updates every `10000` ms

## See also
- entr

## Categories
files, processes

## Links
- More information: https://manned.org/watchexec
- tldr page: https://tldr.inbrowser.app/pages/common/watchexec
