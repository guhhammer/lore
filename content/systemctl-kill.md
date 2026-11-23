# systemctl kill

Send a signal to one or more processes of a unit.

## Usage
- `systemctl kill unit` : send the `SIGTERM` signal to a unit to terminate it
- `systemctl kill -s signal_number|signal_name unit` : send a specific signal to a unit
- `systemctl kill -s 1 --kill-whom main unit` : send a `SIGHUP` signal to only the main process of a unit
- `systemctl kill -s help` : list all available signals

## Categories
processes

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-kill
