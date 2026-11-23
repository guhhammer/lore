# cron

A system scheduler for running jobs or tasks unattended. The command to submit, edit, or delete entries to `cron` is called `crontab`.

## Usage
- `crontab -e` : edit the crontab file for the current user
- `sudo crontab -e -u user` : edit the crontab file for a specific user
- `crontab path/to/file` : replace the current crontab with the contents of the given file
- `crontab -l` : list existing cron jobs for the current user
- `crontab -r` : remove all cron jobs for the current user
- `0 10 * * * command_to_execute` : sample cron job which runs at 10:00 every day (* means any value)
- `*/10 * * * * command_to_execute` : sample cron job which runs a command every 10 minutes
- `30 2 * * Fri /path/to/script.sh` : sample cron job which runs a certain script at 02:30 every Friday

## Categories
system, processes, files

## Links
- More information: https://manned.org/cron
- tldr page: https://tldr.inbrowser.app/pages/common/cron
