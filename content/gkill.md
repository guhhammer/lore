# gkill

This command is an alias of GNU `kill`.

## Usage
- `kill process_id` : terminate a program using the default `SIGTERM` (terminate) signal
- `kill -l` : list available signal names (to be used without the `SIG` prefix)
- `kill -1 process_id` : terminate a program using the `SIGHUP` (hang up) signal. Many daemons will reload instead of terminating
- `kill -2 process_id` : terminate a program using the `SIGINT` (interrupt) signal. This is typically initiated by the user pressing `<Ctrl c>`
- `kill -9 process_id` : signal the operating system to immediately terminate a program (which gets no chance to capture the signal)
- `kill -STOP process_id` : signal the operating system to pause a program until a `SIGCONT` ("continue") signal is received
- `kill -USR1 -group_id` : send a `SIGUSR1` signal to all processes with the given GID (group id)

## Categories
processes, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gkill
