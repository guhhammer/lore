# ionice

Get or set program I/O scheduling class and priority. Scheduling classes: 1 (realtime), 2 (best-effort), 3 (idle). Priority levels: 0 (the highest) - 7 (the lowest).

## Usage
- `ionice -c 1..3 -n 0..7 command` : run a command with the given scheduling class and priority
- `ionice -c 1..3 -p|P|u id` : set I/O scheduling class of a running process with a specific pid, Pgid, or uid
- `ionice -t -n 0..7 -p process_id` : set the priority of a running process, ignoring failure to do so (this can happen due to insufficient privileges or an old kernel version)
- `ionice -p process_id` : print the I/O scheduling class and priority of a running process

## Categories
processes

## Links
- More information: https://manned.org/ionice
- tldr page: https://tldr.inbrowser.app/pages/linux/ionice
