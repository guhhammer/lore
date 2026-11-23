# tlmgr backup

Manage backups of TeX Live packages. The default backup directory is specified by the `backupdir` option, and can be obtained with `tlmgr option`.

## Usage
- `tlmgr backup package1 package2 ...` : make a backup of one or more packages
- `tlmgr backup --all` : make a backup of all packages
- `tlmgr backup package --backupdir path/to/backup_directory` : make a backup to a custom directory
- `tlmgr backup clean package1 package2 ...` : remove a backup of one or more packages
- `tlmgr backup clean --all` : remove all backups

## Categories
packages, files

## Links
- More information: https://www.tug.org/texlive/doc/tlmgr.html#backup
- tldr page: https://tldr.inbrowser.app/pages/common/tlmgr-backup
