# killall

Send kill signal to all instances of a process by name (must be exact name). All signals except `SIGKILL` and `SIGSTOP` can be intercepted by the process, allowing a clean exit.

## Usage
- `killall process_name` : terminate a process using the default `SIGTERM` (terminate) signal
- `killall -l` : list available signal names (to be used without the `SIG` prefix)
- `killall -i process_name` : interactively ask for confirmation before termination
- `killall -2 process_name` : terminate a process using the `SIGINT` (interrupt) signal, which is the same signal sent by pressing `<Ctrl c>`
- `killall -9 process_name` : force kill a process

## Categories
danger, processes

## Links
- More information: https://manned.org/killall
- tldr page: https://tldr.inbrowser.app/pages/common/killall
- Wikipedia: https://en.wikipedia.org/wiki/Killall
