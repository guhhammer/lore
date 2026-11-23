# systemctl restart

Stop and then start one or more systemd units. Can be used in place of `systemctl start` on a stopped unit, but `start` is safer so that a running unit isn't accidentally restarted.

## Usage
- `systemctl restart unit` : restart a unit
- `systemctl restart unit1 unit2 ...` : restart more than one unit
- `systemctl restart unit --user` : restart a user unit

## Categories
system

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-restart
