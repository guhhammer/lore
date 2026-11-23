# tlmgr restore

Restore package backups created with `tlmgr backup`. The default backup directory is specified by the `backupdir` option, and can be obtained with `tlmgr option`.

## Usage
- `tlmgr restore` : list all available backup revisions for all packages
- `tlmgr restore package` : list all available backup revisions for a specific package
- `tlmgr restore package revision` : restore a specific revision of a specific package
- `tlmgr restore --all` : restore the latest revision of all backed-up packages
- `tlmgr restore package revision --backupdir path/to/backup_directory` : restore a package from a custom backup directory
- `tlmgr restore --dry-run package revision` : simulate a restore and print all actions that would be taken

## Categories
packages, files

## Links
- More information: https://www.tug.org/texlive/doc/tlmgr.html#restore
- tldr page: https://tldr.inbrowser.app/pages/common/tlmgr-restore
