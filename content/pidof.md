# pidof

Get the ID of a process using its name.

## Usage
- `pidof bash` : list all process IDs with given name
- `pidof -s bash` : list a single process ID with given name
- `pidof -x script.py` : list process IDs including scripts with given name
- `kill $(pidof name)` : kill all processes with given name

## See also
- pgrep

## Categories
processes

## Links
- More information: https://manned.org/pidof
- tldr page: https://tldr.inbrowser.app/pages/linux/pidof
