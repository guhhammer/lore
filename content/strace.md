# strace

Troubleshooting tool for tracing system calls.

## Usage
- `sudo strace -p process_id` : start tracing a specific process by its PID
- `sudo strace -p process_id -e system_call,system_call2,...` : trace a process and filter output by system call expression
- `sudo strace -p process_id -c` : count time, calls, and errors for each system call and report a summary on program exit
- `sudo strace -p process_id -T -s 32` : show the time spent in every system call and specify the maximum string size to print
- `strace program` : start tracing a program by executing it
- `strace -e trace=file program` : start tracing file operations of a program
- `strace -f -e trace=network -o trace.txt program` : start tracing network operations of a program as well as all its forked and child processes, saving the output to a file

## See also
- fatrace

## Categories
network, processes, users

## Links
- More information: https://manned.org/strace
- tldr page: https://tldr.inbrowser.app/pages/linux/strace
- Wikipedia: https://en.wikipedia.org/wiki/Strace
