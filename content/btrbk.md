# btrbk

Create snapshots and remote backups of btrfs subvolumes.

## Usage
- `sudo btrbk stats` : print statistics about configured subvolumes and snapshots
- `sudo btrbk list` : list configured subvolumes and snapshots
- `sudo btrbk -v dryrun` : print what would happen in a run without making the displayed changes
- `sudo btrbk --progress -v run` : run backup routines verbosely, show progress bar
- `sudo btrbk snapshot` : only create snapshots for configured subvolumes

## Categories
users

## Links
- More information: https://digint.ch/btrbk/doc/readme.html
- tldr page: https://tldr.inbrowser.app/pages/linux/btrbk
