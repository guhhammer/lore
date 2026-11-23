# sessreg

Manage utmpx/wtmpx entries for non-init clients.

## Usage
- `sessreg -w wtmpx-file` : this specifies an alternate wtmpx file, instead of /var/log/wtmp
- `sessreg -u utmpx-file` : this specifies an alternate utmpx file, instead of /var/run/utmp
- `sessreg -l line-name` : this describes the "line" name of the entry
- `sessreg -V` : this option causes the command to print its version and exit

## Categories
files
