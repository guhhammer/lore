# chrt

Manipulate the real-time attributes of a process.

## Usage
- `chrt -p process_id` : display attributes of a process
- `chrt -a -p process_id` : display attributes of all threads of a process
- `chrt -m` : display the min/max priority values that can be used with `chrt`
- `chrt -p priority process_id` : set the scheduling priority of a process
- `chrt --deadline|idle|batch|rr|fifo|other -p priority process_id` : set the scheduling policy of a process

## Categories
processes, desktop

## Links
- More information: https://manned.org/chrt
- tldr page: https://tldr.inbrowser.app/pages/linux/chrt
