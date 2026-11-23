# lbu

Manage `apk` overlay files on a diskless Alpine Linux system. Note: Subcommands like `include` write to `/etc`, which is stored in RAM. You need to run `lbu commit` to save the changes.

## Usage
- `lbu ci` : commit changes to persistent storage (only files in `/etc` by default)
- `lbu st` : list files that would be saved using `commit`
- `lbu diff` : display changes in tracked files that would be saved using `commit`
- `lbu inc path/to/file_or_directory` : include a specific file or directory in the `apk` overlay
- `lbu ex path/to/file_or_directory` : exclude a specific file or directory in `/etc` from the `apk` overlay
- `lbu include|exclude -l` : display the list of manually included/excluded files
- `lbu lb` : list backups (previously created overlays)
- `lbu revert overlay_filename.tar.gz` : revert to a backup overlay

## Categories
version-control, files

## Links
- More information: https://wiki.alpinelinux.org/wiki/Alpine_local_backup
- tldr page: https://tldr.inbrowser.app/pages/linux/lbu
