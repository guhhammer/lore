# dtruss

Trace system calls and kernel activities on `osx` using DTrace. Note: Requires System Integrity Protection to be disabled.

## Usage
- `sudo dtruss -p process_id` : trace a running process by PID
- `sudo dtruss program` : run a program and trace it
- `sudo dtruss -W name` : wait for a process with a matching name and trace it
- `sudo dtruss -eoc -p process_id` : print elapsed time, on-cpu time, and count of each system call
- `sudo dtruss -f -p process_id` : trace a process and follow children
- `sudo dtruss -t syscall -p process_id` : print occurrences of a specific system call

## See also
- strace

## Categories
users, processes, system, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/dtruss.1m.html
- tldr page: https://tldr.inbrowser.app/pages/osx/dtruss
