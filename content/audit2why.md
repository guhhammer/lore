# audit2why

Explain SELinux denials from audit logs. Part of the `policycoreutils-python-utils` package.

## Usage
- `sudo audit2why` : explain the most recent SELinux denial
- `sudo audit2why -i path/to/audit.log` : explain SELinux denials from a specific audit log file
- `sudo ausearch -m avc | audit2why` : explain all SELinux denials from the audit log
- `sudo ausearch -m avc -c service_name | audit2why` : explain denials for a specific service

## See also
- audit2allow
- ausearch
- sealert

## Categories
users, monitoring, packages

## Links
- More information: https://manned.org/audit2why
- tldr page: https://tldr.inbrowser.app/pages/linux/audit2why
