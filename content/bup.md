# bup

Back up system based on the Git packfile format, providing incremental saves and global deduplication.

## Usage
- `bup -d path/to/repository init` : initialize a backup repository in a given local directory
- `bup -d path/to/repository index path/to/directory` : prepare a given directory before taking a backup
- `bup -d path/to/repository save -n backup_name path/to/directory` : back up a directory to the repository specifying its name
- `bup -d path/to/repository ls` : show the backup snapshots currently stored in the repository
- `bup -d path/to/repository restore -C path/to/target_directory backup_name` : restore a specific backup snapshot to a target directory

## Categories
files, packages, version-control

## Links
- More information: https://manned.org/bup
- tldr page: https://tldr.inbrowser.app/pages/common/bup
