# iotop

Display a table of current I/O usage by processes or threads.

## Usage
- `sudo iotop` : start top-like I/O monitor
- `sudo iotop -o` : show only processes or threads actually doing I/O
- `sudo iotop -b` : show I/O usage in non-interactive mode
- `sudo iotop -P` : show only I/O usage of processes (default is to show all threads)
- `sudo iotop -p process_id` : show I/O usage of given PID(s)
- `sudo iotop -u user` : show I/O usage of a given user
- `sudo iotop -a` : show accumulated I/O instead of bandwidth

## Categories
users, monitoring, processes

## Links
- More information: https://manned.org/iotop
- tldr page: https://tldr.inbrowser.app/pages/common/iotop
