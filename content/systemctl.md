# systemctl

Controls systemd, which starts and watches the services of most Linux
systems: start, stop, restart a service, turn it on at boot, see its state.

## Usage
- `systemctl status nginx` : is it running, and its last log lines
- `sudo systemctl restart nginx` : restart a service
- `sudo systemctl enable --now nginx` : start it now and at every boot
- `sudo systemctl disable nginx` : no longer start it at boot
- `systemctl --failed` : every service that failed
- `journalctl -u nginx -e` : a service's full log, newest at the end

## Categories
system

## Links
- systemd manual: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html
