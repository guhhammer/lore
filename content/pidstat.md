# pidstat

Show system resource usage, including CPU, memory, IO etc.

## Usage
- `pidstat 2 10` : show CPU statistics at a 2 second interval for 10 times
- `pidstat -r` : show page faults and memory utilization
- `pidstat -d` : show input/output usage per process ID
- `pidstat -p process_id` : show information on a specific PID
- `pidstat -C "fox|bird" -r -p ALL` : show memory statistics for all processes whose command name include "fox" or "bird"

## Categories
monitoring, hardware, processes

## Links
- More information: https://manned.org/pidstat
- tldr page: https://tldr.inbrowser.app/pages/linux/pidstat
