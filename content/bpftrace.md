# bpftrace

High-level tracing language for Linux eBPF.

## Usage
- `sudo bpftrace -l` : list all available probes
- `sudo bpftrace -e 'tracepoint:raw_syscalls:sys_enter { @[comm] = count(); }'` : run a one-liner program (e.g. syscall count by program)
- `sudo bpftrace path/to/file` : run a program from a file
- `sudo bpftrace -e 'tracepoint:raw_syscalls:sys_enter /pid == 123/ { @[comm] = count(); }'` : trace a program by PID
- `sudo bpftrace -d -e 'one_line_program'` : simulate running the program and display the output in eBPF format
- `bpftrace -V` : display version

## Categories
users, files

## Links
- More information: https://github.com/bpftrace/bpftrace/blob/master/man/adoc/bpftrace.adoc
- tldr page: https://tldr.inbrowser.app/pages/linux/bpftrace
