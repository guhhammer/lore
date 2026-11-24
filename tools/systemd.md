# systemd

systemd starts and watches services on most Linux systems. systemctl controls them, journalctl reads their logs and timers replace cron jobs.

## Tasks
### Run a service
- `systemctl status NAME` : is it running, and the last log lines
- `sudo systemctl start NAME` : start it now
- `sudo systemctl stop NAME` : stop it
- `sudo systemctl restart NAME` : stop and start
- `sudo systemctl reload NAME` : reread its config without stopping
- `sudo systemctl enable --now NAME` : start now and at every boot
- `sudo systemctl disable --now NAME` : stop now and at no boot
### Look
- `systemctl list-units --type=service` : running services
- `systemctl --failed` : units that failed
- `systemctl is-active NAME` : active or not, for scripts
- `systemctl cat NAME` : show the unit file in use
- `systemctl list-dependencies NAME` : what it needs
### Write a service
- `sudo systemctl edit --full NAME` : edit a unit file
- `sudo systemctl edit NAME` : add overrides without touching the original
- `sudo systemctl daemon-reload` : reread unit files after changing them
- `systemd-analyze verify FILE.service` : check a unit file for mistakes
- `systemctl --user enable --now NAME` : a service for your own user
### Logs
- `journalctl -u NAME` : logs of one service
- `journalctl -u NAME -f` : follow them live
- `journalctl -u NAME --since "1 hour ago"` : only recent lines
- `journalctl -b` : everything since this boot
- `journalctl -p err -b` : errors of this boot
- `journalctl --disk-usage` : space the logs use
- `sudo journalctl --vacuum-time=2weeks` : delete logs older than two weeks (deletes them)
### Timers and power
- `systemctl list-timers` : scheduled timers and when they run next
- `sudo systemctl enable --now NAME.timer` : turn a timer on
- `systemd-analyze blame` : which units slowed the boot
- `systemctl reboot` : restart the machine

## Notes
- After editing a unit file always run `systemctl daemon-reload`.
- Unit files you write belong in `/etc/systemd/system/`; the ones in `/usr/lib/systemd/system/` come from packages.
- `enable` only sets boot behaviour; `start` runs it now; `enable --now` does both.
- A timer `NAME.timer` starts the service `NAME.service` of the same name.

## See also
- cron
- journalctl
- systemd-analyze
- nginx

## Links
- systemctl: https://man7.org/linux/man-pages/man1/systemctl.1.html
- Unit files: https://man7.org/linux/man-pages/man5/systemd.service.5.html

## Categories
services system
