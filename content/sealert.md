# sealert

Analyze and explain SELinux AVC denial messages. Part of the `setroubleshoot-server` package.

## Usage
- `sudo sealert -a /var/log/audit/audit.log` : analyze all recent SELinux denials
- `sudo sealert -l alert_id` : analyze a specific alert ID from system logs
- `sudo sealert -b` : display a summary of recent SELinux alerts
- `sudo tail -f /var/log/audit/audit.log | sealert -l -` : monitor audit log in real-time for new alerts

## See also
- audit2why
- ausearch
- audit2allow

## Categories
monitoring, users, packages

## Links
- More information: https://manned.org/sealert
- tldr page: https://tldr.inbrowser.app/pages/linux/sealert
