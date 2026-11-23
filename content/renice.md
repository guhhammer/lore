# renice

Alter the scheduling priority/niceness of running processes. Niceness values range from -20 (most favorable to the process) to 19 (least favorable to the process).

## Usage
- `renice -n 3 -p process_id` : increase/decrease the priority of a running process
- `renice -n -4 -u user_id|username` : increase/decrease the priority of all processes owned by a user
- `renice -n 5 -g process_group` : increase/decrease the priority of all processes that belong to a process group

## See also
- nice

## Categories
processes

## Links
- More information: https://manned.org/renice.1p
- tldr page: https://tldr.inbrowser.app/pages/common/renice
