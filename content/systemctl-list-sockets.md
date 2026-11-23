# systemctl list-sockets

List active socket units currently in memory, ordered by listening address.

## Usage
- `systemctl list-sockets` : list active socket units currently in memory
- `systemctl list-sockets --show-types` : list active socket units with their socket types
- `systemctl list-sockets -a` : list all socket units, including inactive and failed ones
- `systemctl list-sockets --state active|inactive|failed|...` : list socket units filtered by state
- `systemctl list-sockets pattern1 pattern2 ...` : list socket units matching a name pattern

## See also
- systemctl list-units
- systemctl list-unit-files

## Categories
network, monitoring

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-list-sockets
