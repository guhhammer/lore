# systemctl list-paths

List path units currently in memory, ordered by path.

## Usage
- `systemctl list-paths` : show all path units currently in memory
- `systemctl list-paths pattern` : list path units matching specific wildcard pattern i.e. a `glob`
- `systemctl list-paths pattern_1 pattern_2 ...` : list path units that match with multiple patterns
- `systemctl list-paths -a` : show all path units, including inactive ones
- `systemctl list-paths --state state` : filter path units by state
- `systemctl list-paths --show-types` : also show unit types in the output

## Categories
files, monitoring

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-list-paths
