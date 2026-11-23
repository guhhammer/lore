# logrotate

Rotate, compress, and mail system logs.

## Usage
- `logrotate path/to/logrotate.conf --force` : trigger a run manually
- `logrotate path/to/logrotate.conf --mail /usr/bin/mail_command` : run using a specific command to mail reports
- `logrotate path/to/logrotate.conf --state /dev/null` : run without using a state (lock) file
- `logrotate path/to/logrotate.conf --skip-state-lock` : run and skip the state (lock) file check
- `logrotate path/to/logrotate.conf --log path/to/log_file` : tell `logrotate` to log verbose output into the log file

## Categories
files, archives, monitoring

## Links
- More information: https://manned.org/logrotate
- tldr page: https://tldr.inbrowser.app/pages/linux/logrotate
