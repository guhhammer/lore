# borg

Deduplicating backup tool. Create local or remote backups that are mountable as filesystems.

## Usage
- `borg init -e repokey|keyfile|none|... path/to/repo_directory` : initialize a repository
- `borg create --progress path/to/repo_directory::Monday path/to/source_directory` : back up a directory into the repository, creating an archive called "Monday"
- `borg list path/to/repo_directory` : list all archives in a repository
- `borg extract user@host:path/to/repo_directory::Monday path/to/target_directory --exclude '*.ext'` : extract a specific directory from the "Monday" archive in a remote repository, excluding all `*.ext` files
- `borg prune --keep-within 7d --list path/to/repo_directory` : prune a repository by deleting all archives older than 7 days, listing changes
- `borg mount path/to/repo_directory::Monday path/to/mount_point` : mount a repository as a FUSE filesystem
- `borg create --help` : display help on creating archives

## Categories
files, packages, archives

## Links
- More information: https://borgbackup.readthedocs.io/en/stable/usage/general.html
- tldr page: https://tldr.inbrowser.app/pages/common/borg
