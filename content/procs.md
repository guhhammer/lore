# procs

Display information about the active processes.

## Usage
- `procs` : list all processes showing the PID, user, CPU usage, memory usage, and the command which started them
- `procs --tree` : list all processes as a tree
- `procs zsh` : list information about processes, if the commands which started them contain `zsh`
- `procs --sorta|--sortd cpu` : list information about all processes sorted by CPU time in ascending or descending order
- `procs --or PID|command|user 41 firefox` : list information about processes with either a PID, command, or user containing `41` or `firefox`
- `procs --and 41 zsh` : list information about processes with both PID `41` and a command or user containing `zsh`

## Categories
processes, monitoring, users

## Links
- More information: https://github.com/dalance/procs/blob/master/man/procs.1.adoc
- tldr page: https://tldr.inbrowser.app/pages/common/procs
