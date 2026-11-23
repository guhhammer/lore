# systemd-run

Run programs in transient scope units, service units, or path-, socket-, or timer-triggered service units.

## Usage
- `sudo systemd-run command argument1 argument2 ...` : start a transient service
- `systemd-run --user command argument1 argument2 ...` : start a transient service under the service manager of the current user (no privileges)
- `sudo systemd-run -u name --description string command argument1 argument2 ...` : start a transient service with a custom unit name and description
- `sudo systemd-run -r -E name=value command argument1 argument2 ...` : start a transient service that does not get cleaned up after it terminates with a custom environment variable
- `sudo systemd-run --on-calendar=calendar_event command argument1 argument2 ...` : start a transient timer that periodically runs its transient service (see `man systemd.time` for calendar event format)
- `systemd-run -r --pty command` : share the terminal with the program (allowing interactive input/output) and make sure the execution details remain after the program exits
- `systemd-run -p MemoryMax=memory_in_bytes -p CPUQuota=percentage_of_cpu_time% --wait command` : set properties (e.g. CPUQuota, MemoryMax) of the process and wait until it exits
- `command1 | systemd-run -P command2 | command3` : use the program in a shell pipeline

## Categories
system, users, network

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-run.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-run
