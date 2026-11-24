# restic

A fast and secure backup program.

## Usage
- `restic init -r path/to/repository` : initialize a backup repository in the specified local directory
- `restic -r path/to/repository backup path/to/directory` : back up a directory to the repository
- `restic -r path/to/repository snapshots` : show backup snapshots currently stored in the repository
- `restic -r path/to/repository restore latest|snapshot_id --target path/to/target` : restore a specific backup snapshot to a target directory
- `restic -r path/to/repository restore snapshot_id --target path/to/target --include path/to/restore` : restore a specific path from a specific backup to a target directory
- `restic forget --keep-last 1 --prune` : clean up the repository and keep only the most recent snapshot of each unique backup

## Categories
files, packages, security

## Links
- More information: https://restic.readthedocs.io/en/stable/manual_rest.html#usage-help
- tldr page: https://tldr.inbrowser.app/pages/common/restic
- Wikipedia: https://en.wikipedia.org/wiki/Restic
