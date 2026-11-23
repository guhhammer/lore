# flock

Manage file locks from shell scripts. It can be used to ensure that only one instance of a command is running.

## Usage
- `flock path/to/lock.lock command` : run a command with a file lock as soon as the lock is available
- `flock -n path/to/lock.lock command` : run a command with a file lock, or exit if the lock is currently being held (with exit code 1)
- `flock -n -E 123 path/to/lock.lock command` : run a command with a file lock, or exit with a specific error code if the lock is currently being held
- `flock -w 10 path/to/lock.lock command` : run a command with a file lock, waiting up to 10 seconds for the lock to be available before giving up
- `flock path/to/backup.lock tar -cvf path/to/backup.tar path/to/data/` : back up a bunch of files, waiting for the previous `tar` command to finish if it's still running elsewhere and holding the same lock file (can be used in a `cron` job that runs often)

## Categories
files, archives, shell

## Links
- More information: https://manned.org/flock
- tldr page: https://tldr.inbrowser.app/pages/linux/flock
