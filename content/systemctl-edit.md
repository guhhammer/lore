# systemctl edit

Edit systemd unit files.

## Usage
- `sudo systemctl edit unit_file` : overlay a unit file non-destructively
- `sudo systemctl edit unit_file -l` : edit a unit file
- `sudo systemctl edit unit_file -lf` : create a new unit file
- `systemctl edit unit_file --user` : overlay a user unit file

## See also
- systemctl revert

## Categories
users, files, system

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-edit
