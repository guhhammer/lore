# rdiff-backup

Local/remote mirror and incremental backup tool.

## Usage
- `rdiff-backup backup path/to/source path/to/backup` : back up `path/to/source` to `path/to/backup`
- `rdiff-backup list increments repository` : list incremental backups in repository (location path, local, or remote)
- `rdiff-backup restore path/to/backup path/to/destination` : restore from most recent backup
- `rdiff-backup restore --at 3D path/to/backup path/to/destination` : restore backed up files as they were 3 days ago

## Categories
files

## Links
- More information: https://rdiff-backup.net/rdiff-backup.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/rdiff-backup
