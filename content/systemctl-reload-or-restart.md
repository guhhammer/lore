# systemctl reload-or-restart

Reload `systemd` unit(s) otherwise restart them.

## Usage
- `systemctl reload-or-restart unit` : reload or restart a unit
- `systemctl reload-or-restart pattern` : reload or restart multiple units matching a pattern
- `systemctl reload-or-restart unit --no-block` : run the command without waiting for the operation to complete
- `systemctl reload-or-restart unit --user` : apply the command only to user units

## Categories
system

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-reload-or-restart
