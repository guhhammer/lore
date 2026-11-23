# exim_lock

Mailbox maintenance.

## Usage
- `exim_lock [options]mailbox-file` : the general form, as the manual gives it
- `exim_lock -lockfile` : create a lock file before opening the mailbox
- `exim_lock -mbx` : lock the mailbox using MBX rules
- `exim_lock -q` : suppress verification output
- `exim_lock -v` : generate verbose output
