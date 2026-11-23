# docker top

This command is an alias of `docker container top`.

## Usage
- `top` : start `top`
- `top -i` : do not show any idle or zombie processes
- `top -u username` : show only processes owned by given user
- `top -o field_name` : sort processes by a field
- `top -Hp process_id` : show the individual threads of a given process
- `top -p $(pgrep -d ',' process_name)` : show only the processes with the given PID(s), passed as a comma-separated list. (Normally you wouldn't know PIDs off hand. This example picks the PIDs from the process name)
- `<?>` : display help about interactive commands

## Categories
containers, processes, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/docker-top
