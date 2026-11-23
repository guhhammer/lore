# systemctl is-failed

Check if one or more systemd units have failed.

## Usage
- `systemctl is-failed` : check if there are any failed units
- `systemctl is-failed unit1 unit2 ...` : check if a unit or multiple units have failed
- `systemctl is-failed unit -q` : suppress output and return only the exit code
- `systemctl is-failed unit --user` : check if a user unit has failed

## See also
- systemctl is-active
- systemctl status

## Categories
system

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-is-failed
