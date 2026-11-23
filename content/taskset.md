# taskset

Get or set a process' CPU affinity or start a new process with a defined CPU affinity.

## Usage
- `taskset -p -c process_id` : get a running process' CPU affinity by PID
- `taskset -p -c cpu_id process_id` : set a running process' CPU affinity by PID
- `taskset -c cpu_id command` : start a new process with affinity for a single CPU
- `taskset -c cpu_id_1,cpu_id_2,cpu_id_3,...` : start a new process with affinity for multiple non-sequential CPUs
- `taskset -c cpu_id_1-cpu_id_4` : start a new process with affinity for CPUs 1 through 4

## Categories
processes, hardware

## Links
- More information: https://manned.org/taskset
- tldr page: https://tldr.inbrowser.app/pages/linux/taskset
