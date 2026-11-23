# systemctl reenable

Re-enable one or more units. Used when targets of a service change.

## Usage
- `systemctl reenable unit` : re-enable a unit, restoring its default symlinks
- `systemctl reenable unit1 unit2 ...` : re-enable multiple units at once
- `systemctl reenable unit --now` : re-enable a unit and start it immediately

## Categories
system

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-reenable
