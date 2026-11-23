# setsid

Run a program in a new session if the calling process is not a process group leader. The created session is by default not controlled by the current terminal.

## Usage
- `setsid program` : run a program in a new session
- `setsid program >/dev/null 2>&1` : run a program in a new session discarding the resulting output and error
- `setsid -f program` : run a program creating a new process
- `setsid -w program` : return the exit code of a program as the exit code of setsid when the program exits
- `setsid -c program` : run a program in a new session setting the current terminal as the controlling terminal

## Categories
processes, shell, users

## Links
- More information: https://manned.org/setsid
- tldr page: https://tldr.inbrowser.app/pages/linux/setsid
