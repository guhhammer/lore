# numactl

Control NUMA policy for processes or shared memory.

## Usage
- `numactl --cpunodebind=0 --membind=0,1 -- command command_arguments` : run a command on node 0 with memory allocated on node 0 and 1
- `numactl --physcpubind=+0-4,8-12 -- command command_arguments` : run a command on CPUs (cores) 0-4 and 8-12 of the current cpuset
- `numactl --interleave=all -- command command_arguments` : run a command with its memory interleaved on all CPUs

## Categories
monitoring, processes

## Links
- More information: https://manned.org/numactl
- tldr page: https://tldr.inbrowser.app/pages/linux/numactl
