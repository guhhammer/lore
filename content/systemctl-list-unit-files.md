# systemctl list-unit-files

List installed unit files and their enablement states.

## Usage
- `systemctl list-unit-files` : list installed unit files and their states
- `systemctl list-unit-files --state enabled|disabled|static|...` : filter by state
- `systemctl list-unit-files -t service|socket|timer|...` : filter by unit type
- `systemctl list-unit-files 'sshd*'` : filter by a name pattern
- `systemctl list-unit-files --no-pager` : print output directly to `stdout`
- `systemctl list-unit-files --no-legend` : print output without headers or footers

## See also
- systemctl list-units

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-list-unit-files
