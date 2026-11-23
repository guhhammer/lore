# ufw logging

Configure logging of firewall events. Note: Logging levels `high` and `full` may quickly fill up disk space.

## Usage
- `sudo ufw logging on` : enable logging with the default level (low)
- `sudo ufw logging off` : disable logging
- `sudo ufw logging low|medium|high|full` : set the logging level
- `sudo ufw allow|deny|reject|limit log|log-all 22/tcp` : log new connections (`log`) or all packets (`log-all`) matching a rule

## Categories
monitoring, users, disks

## Links
- More information: https://manned.org/ufw
- tldr page: https://tldr.inbrowser.app/pages/linux/ufw-logging
