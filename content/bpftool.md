# bpftool

Inspect and manipulate eBPF programs and maps in a simple way.

## Usage
- `bpftool prog list` : list information about loaded `eBPF` programs
- `bpftool net list` : list `eBPF` program attachments in the kernel networking subsystem
- `bpftool link list` : list all active links
- `bpftool perf list` : list all `raw_tracepoint`, `tracepoint`, `kprobe` attachments in the system
- `bpftool btf list` : list `BPF Type Format (BTF)` data
- `bpftool map list` : list information about loaded maps
- `bpftool feature probe dev eth0` : probe a network device "eth0" for supported `eBPF` features
- `bpftool batch file myfile` : run commands in batch mode from a file

## Categories
files, network

## Links
- More information: https://manned.org/bpftool
- tldr page: https://tldr.inbrowser.app/pages/linux/bpftool
