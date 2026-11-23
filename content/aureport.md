# aureport

Generate summary reports of auditd logs.

## Usage
- `sudo aureport` : display a summary of auditd events
- `sudo aureport -l` : generate a summary of login events
- `sudo aureport -s` : list all syscall reports
- `sudo aureport -x` : generate a summary of executable events
- `sudo aureport -ts start_time -te end_time` : show a summary of events for a specific time range
- `sudo aureport -t` : list all audit files and the time range of events they cover
- `aureport --help` : display help

## Categories
users

## Links
- More information: https://manned.org/aureport
- tldr page: https://tldr.inbrowser.app/pages/linux/aureport
