# top

Display dynamic real-time information about running processes.

## Usage
- `top` : start `top`
- `top -i` : do not show any idle or zombie processes
- `top -u username` : show only processes owned by given user
- `top -o field_name` : sort processes by a field
- `top -Hp process_id` : show the individual threads of a given process
- `top -p $(pgrep -d ',' process_name)` : show only the processes with the given PID(s), passed as a comma-separated list. (Normally you wouldn't know PIDs off hand. This example picks the PIDs from the process name)
- `<?>` : display help about interactive commands

## See also
- htop
- atop
- glances
- btop
- btm

## Categories
processes, monitoring

## Links
- More information: https://manned.org/top
- tldr page: https://tldr.inbrowser.app/pages/linux/top
