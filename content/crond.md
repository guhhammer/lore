# crond

Daemon to execute scheduled commands from crontab files.

## Usage
- `crond` : start daemon in the background and check for scheduled commands
- `crond -n` : start daemon in the foreground and check for scheduled commands
- `crond -s` : send job output from the daemon to the system log
- `crond -p` : override default limitations and accept custom crontables
- `crond -P` : inherit crontab file path from environment settings

## Categories
system

## Links
- More information: https://manned.org/crond
- tldr page: https://tldr.inbrowser.app/pages/linux/crond
