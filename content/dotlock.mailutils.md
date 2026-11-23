# dotlock.mailutils

Lock mail spool files.

## Usage
- `dotlock.mailutils [OPTION...] FILE` : the general form, as the manual gives it
- `dotlock.mailutils -r` : retry the lock a few times
- `dotlock.mailutils -t` : delay between two successive locking attempts (in seconds)
- `dotlock.mailutils --debug-level=LEVEL` : set Mailutils debugging level
- `dotlock.mailutils --config-file=FILE` : load this configuration file; implies --no-config

## Categories
files

## Links
- Documentation: https://www.gnu.org/software/mailutils/manual/index.html
