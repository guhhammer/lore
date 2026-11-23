# systemd-cat

Connect a pipeline or program's output streams with the systemd journal.

## Usage
- `systemd-cat command` : write the output of the specified command to the journal (both output streams are captured)
- `command | systemd-cat` : write the output of a pipeline to the journal (`stderr` stays connected to the terminal)
- `command | systemd-cat -t id` : use the specified identifier (default: `cat` when reading from a pipeline, executable name otherwise)
- `systemd-cat -p emerg|alert|crit|err|warning|notice|info|debug command` : use the specified default priority level for all logged messages
- `systemd-cat --stderr-priority emerg|alert|crit|err|warning|notice|info|debug command` : use the specified default priority level for the logged messages from the command's `stderr`

## Categories
system, processes

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-cat.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-cat
