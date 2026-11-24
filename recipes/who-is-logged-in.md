# who is logged in

See who is on this machine now, who was before, and the failed attempts.

## Steps
- `who` : the users logged in now, and from where
- `w` : the same, with what each one is running
- `last -n 20` : the last 20 logins
- `sudo lastb -n 20` : the last 20 failed logins
- `journalctl -u ssh --since today` : today's ssh logins and attempts (the service is `sshd` on Fedora and Arch)
- `loginctl list-sessions` : the sessions systemd knows

## Notes
- Many failed logins from one address usually mean someone is guessing passwords: see fail2ban.
- macOS: `who`, `w` and `last` work the same.

## See also
- who
- w
- last
- journalctl

## Categories
users security
