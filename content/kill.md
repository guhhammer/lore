# kill

Send a signal to a process, usually related to stopping the process. All signals except for `SIGKILL` and `SIGSTOP` can be intercepted by the process to perform a clean exit.

## Usage
- `kill process_id` : terminate a program using the default `SIGTERM` (terminate) signal
- `kill -l` : list available signal names (to be used without the `SIG` prefix)
- `kill -1 process_id` : terminate a program using the `SIGHUP` (hang up) signal. Many daemons will reload instead of terminating
- `kill -2 process_id` : terminate a program using the `SIGINT` (interrupt) signal. This is typically initiated by the user pressing `<Ctrl c>`
- `kill -9 process_id` : signal the operating system to immediately terminate a program (which gets no chance to capture the signal)
- `kill -STOP process_id` : signal the operating system to pause a program until a `SIGCONT` ("continue") signal is received
- `kill -USR1 -group_id` : send a `SIGUSR1` signal to all processes with the given GID (group id)

## Categories
danger, processes, shell

## Links
- More information: https://manned.org/kill.1posix
- tldr page: https://tldr.inbrowser.app/pages/common/kill
