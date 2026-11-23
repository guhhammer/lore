# systemctl add-wants

Add `Wants` dependencies to a target for one or more units.

## Usage
- `systemctl add-wants target unit` : add a `Wants` dependency from a target to a unit
- `systemctl add-wants target unit1 unit2 ...` : add multiple `Wants` dependencies at once
- `systemctl add-wants target unit --user` : add a user-level `Wants` dependency

## Categories
packages

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-add-wants
