# why won't a service start

Read what a systemd service said when it failed, check its unit file and
its configuration, then start it again.

## Steps
- `systemctl status NAME` : its state, its last log lines and its exit code
- `journalctl -u NAME -b -n 50 --no-pager` : its last 50 log lines since the machine started
- `systemctl cat NAME` : the unit file it runs: the command, the user, the paths
- `systemd-analyze verify NAME.service` : look for mistakes in the unit file
- `sudo ss -tulpn` : see whether another program already holds its port
- `sudo systemctl daemon-reload` : after editing a unit file, make systemd read it again
- `sudo systemctl restart NAME` : start it again, then look at its status

## Notes
- Put the service's name in place of NAME: `nginx`, `ssh`, `docker`.
- Many services check their own configuration: `sudo nginx -t`, `sudo apachectl configtest`, `sudo sshd -t`.
- "Permission denied" in the log often means a file or folder it needs belongs to another user.
- macOS uses launchd: `launchctl list | grep NAME`, and its logs with `log show --last 10m`.

## See also
- systemctl
- journalctl
- systemd-analyze

## Categories
system services
