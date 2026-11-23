# systemctl list-dependencies

Show a unit's dependency tree in systemd.

## Usage
- `systemctl list-dependencies` : show the dependency tree of `default.target`
- `systemctl list-dependencies unit` : show the dependency tree of a specific unit
- `systemctl list-dependencies unit -a` : include all dependency types (not only `Requires=` and `Wants=`)
- `systemctl list-dependencies unit -t service|socket|target|mount|...` : limit the tree to a specific unit type
- `systemctl list-dependencies unit --reverse` : reverse the direction to show units that depend on the specified unit
- `systemctl list-dependencies unit --no-legend` : print output without headers or footers (for scripts)

## See also
- systemctl list-units

## Categories
packages, system

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-list-dependencies
