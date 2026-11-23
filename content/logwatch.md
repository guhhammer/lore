# logwatch

Summarize many different logs for common services (e.g. apache, pam_unix, sshd, etc.) in a single report.

## Usage
- `logwatch --range yesterday|today|all|help --detail low|medium|others` : analyze logs for a range of dates at a certain level of detail
- `logwatch --range all --service apache|pam_unix|...` : restrict report to only include information for a selected service

## Categories
system

## Links
- More information: https://manned.org/logwatch
- tldr page: https://tldr.inbrowser.app/pages/linux/logwatch
