# ifquery

Parse interface configuration.

## Usage
- `ifquery [-nv] [--verbose] [-i FILE|--interfaces=FILE] [--state-dir=DIR] [--allow CLASS] IFACE...` : the general form, as the manual gives it
- `ifquery --state [--state-dir=DIR] [--allow CLASS] [-a|IFACE...]` : another form of the same command
- `ifquery -f` : force configuration or deconfiguration of the interface
- `ifquery --ignore-errors` : if any of the commands of scripts fails, continue
- `ifquery -h` : show summary of options
- `ifquery --state-dir=DIR` : keep interface state in DIR instead of in /run/network

## Categories
network
