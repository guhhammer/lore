# schedule a command with cron

Run a command every day, every hour or every few minutes, and see what is scheduled.

## Steps
- `crontab -e` : edit your schedule; a line is minute hour day month weekday command
- `crontab -l` : show your schedule
- `sudo crontab -u NAME -l` : another user's
- `ls /etc/cron.d /etc/cron.daily /etc/cron.hourly` : the system's scheduled jobs
- `systemctl list-timers` : systemd's timers, the other kind of schedule
- `grep CRON /var/log/syslog` : check that a job ran (Debian, Ubuntu)

## Notes
- `*/5 * * * * /home/me/backup.sh` runs every 5 minutes; `0 3 * * * …` every day at 3:00.
- cron sees a new crontab by itself: no restart needed.
- cron runs with almost no PATH: write full paths, and send the output to a file to see errors.

## See also
- crontab
- systemctl

## Categories
system services
