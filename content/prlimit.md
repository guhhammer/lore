# prlimit

Get or set process resource soft and hard limits. Given a process ID and one or more resources, prlimit tries to retrieve and/or modify the limits.

## Usage
- `prlimit` : display limit values for all current resources for the running parent process
- `prlimit -p process_id_number` : display limit values for all current resources of a specified process
- `prlimit -n10 command` : run a command with a custom number of open files limit

## Categories
processes

## Links
- More information: https://manned.org/prlimit
- tldr page: https://tldr.inbrowser.app/pages/linux/prlimit
