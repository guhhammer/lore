# killall5

Sends a signal to every process outside its own session, the way the System V init scripts did at shutdown.

## Usage
- `killall5 -15` : ask every such process to terminate (SIGTERM)
- `killall5 -9` : kill every such process without letting it clean up
- `killall5 -9 -o 1234` : kill all of them except the process with that id

## Notes
Run as root this can kill nearly everything on the machine, including your login. Do not try it on a desktop.

## Categories
processes, system
