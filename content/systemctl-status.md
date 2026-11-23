# systemctl status

Display the status of systemd units.

## Usage
- `systemctl status unit.service|timer|socket|target|...` : show the status of a systemd unit
- `systemctl status --failed` : show the status of failed units
- `systemctl status` : list all running services
- `systemctl status -a` : list all units in the system
- `systemctl status -t service|timer|socket|target|...` : list all units of a specific type
- `systemctl status --state active|inactive|failed` : list all units with a specific state
- `systemctl status unit --user` : show the status of a user unit

## Categories
system, desktop, network

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-status
