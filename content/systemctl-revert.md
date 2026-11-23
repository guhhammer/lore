# systemctl revert

Revert unit files to their vendor versions. Undoes the effects of `edit`, `enable`, `disable`, `set-property`, and `mask`.

## Usage
- `systemctl revert unit1 unit2 ...` : revert unit files to their default settings
- `systemctl revert unit --user` : revert a user unit file

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-revert
