# truss

Troubleshooting tool for tracing system calls. SunOS equivalent of `strace`.

## Usage
- `truss -f program` : start tracing a program by executing it, following all child processes
- `truss -p process_id` : start tracing a specific process by its PID
- `truss -a -e program` : start tracing a program by executing it, showing arguments and environment variables
- `truss -c -p process_id` : count time, calls, and errors for each system call and report a summary on program exit
- `truss -p process_id -t system_call_name` : trace a process filtering output by system call

## Notes
A SunOS (Solaris) command: this page is for reference, it does not run on Linux or macOS.

## Categories
processes, sunos

## Links
- More information: https://www.unix.com/man-page/sunos/1/truss
- tldr page: https://tldr.inbrowser.app/pages/sunos/truss
