# cron

cron runs commands on a schedule. Each user has a crontab, a file with one line per job.

## Tasks
### Edit and list
- `crontab -e` : edit your crontab
- `crontab -l` : list your jobs
- `crontab -r` : delete your whole crontab (deletes every job)
- `sudo crontab -u USER -e` : edit another user's crontab
- `sudo crontab -l -u root` : list root's jobs
### Schedules (the five fields: minute hour day month weekday)
- `* * * * * COMMAND` : every minute
- `*/5 * * * * COMMAND` : every 5 minutes
- `0 * * * * COMMAND` : at the start of every hour
- `30 2 * * * COMMAND` : every day at 02:30
- `0 9 * * 1-5 COMMAND` : weekdays at 09:00
- `0 0 1 * * COMMAND` : the first of every month at midnight
- `@reboot COMMAND` : once, when the machine starts
- `@daily COMMAND` : once a day (also @hourly, @weekly, @monthly)
### Write a good job
- `0 3 * * * /usr/bin/backup.sh >> /var/log/backup.log 2>&1` : keep the output in a log
- `*/10 * * * * /usr/bin/flock -n /tmp/job.lock /path/job.sh` : never run two copies at once
- `MAILTO=you@example.org` : send job output to an address (a line at the top of the crontab)
- `PATH=/usr/local/bin:/usr/bin:/bin` : set PATH for the jobs (a line at the top)
- `0 4 * * * cd /path && ./run.sh` : jobs start in your home: change folder first
### System jobs
- `ls /etc/cron.d /etc/cron.daily` : where packages and admins put system jobs
- `sudo nano /etc/cron.d/NAME` : a system job file; it has a user column after the schedule
- `grep CRON /var/log/syslog` : see runs in the log (Debian, Ubuntu)
- `journalctl -u cron` : the same on systems with journald
- `systemctl status cron` : is the daemon running (crond on Fedora)
### Check a schedule
- `date` : compare with the system's clock and timezone

## Notes
- cron runs with a bare PATH and no profile: use full paths to commands.
- `%` has a special meaning in a crontab line: escape it as `\%`, for example in `date +\%F`.
- Jobs run in the server's timezone, which may not be yours.
- `crontab -r` has no confirmation and no undo: save with `crontab -l > FILE` first.
- For jobs that need logging or dependencies, a systemd timer is often a better fit.

## See also
- systemd
- at
- anacron

## Links
- crontab(5): https://man7.org/linux/man-pages/man5/crontab.5.html
- crontab(1): https://man7.org/linux/man-pages/man1/crontab.1.html

## Categories
services system
