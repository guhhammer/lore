# systemctl whoami

Show units that processes belong to. If no PID is specified, shows the unit the `systemctl` command itself is invoked in.

## Usage
- `systemctl whoami` : show the unit of the current shell (where `systemctl` is running)
- `systemctl whoami --user` : show the unit of the current shell in user service manager (services managed for your login session)
- `systemctl whoami process_id` : show the unit a specific process belongs to
- `systemctl whoami process_id1 process_id2 ...` : show the units for multiple processes

## Categories
processes, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-whoami
