# rrsync

A script to setup restricted rsync users via ssh logins.

## Usage
- `rrsync -ro` : allow only reading from the DIR. Implies -no-del and -no-lock
- `rrsync -wo` : allow only writing to the DIR
- `rrsync -no-del` : disable rsync's --delete* and --remove* options
- `rrsync -no-lock` : avoid the single-run (per-user) lock check

## Categories
network, users, development

## Links
- Documentation: https://download.samba.org/pub/rsync/rrsync.1
