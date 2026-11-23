# systemctl add-requires

Add `Requires` dependencies to a target for one or more units.

## Usage
- `systemctl add-requires target unit` : add a `Requires` dependency from a target to a unit
- `systemctl add-requires target unit1 unit2 ...` : add multiple `Requires` dependencies at once
- `systemctl add-requires target unit --user` : add a user-level `Requires` dependency

## Categories
packages

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-add-requires
