# systemctl list-units

List units that systemd currently has in memory.

## Usage
- `systemctl list-units` : list units which are active, have pending jobs, or have failed
- `systemctl list-units -a` : list all units, including inactive ones
- `systemctl list-units -t service|socket|timer|...` : filter by unit type
- `systemctl list-units --state running|listening|dead|...` : filter by state
- `systemctl list-units 'systemd*'` : filter by a name pattern
- `systemctl list-units --no-pager` : print output directly to `stdout`
- `systemctl list-units --no-legend` : print output without headers or footers (for scripts)

## See also
- systemctl list-unit-files

## Categories
system, monitoring

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-list-units
