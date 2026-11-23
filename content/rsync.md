# rsync

Transfer files either to or from a remote host (but not between two remote hosts), by default using SSH. To specify a remote path, use `user@host:path/to/file_or_directory`.

## Usage
- `rsync path/to/source path/to/destination` : transfer a file (use `--dry-run` to simulate the transfer)
- `rsync -a path/to/source path/to/destination` : use archive mode (recursively copy directories, copy symlinks without resolving, and preserve permissions, ownership, and modification times)
- `rsync -zvhP path/to/source path/to/destination` : compress the data as it is sent to the destination, display verbose and human-readable progress, and keep partially transferred files if interrupted
- `rsync -r --fsync path/to/source path/to/destination` : recursively copy directories and ensure each file is fully committed to disk rather than remaining in RAM
- `rsync -r path/to/source/ path/to/destination` : transfer directory contents, but not the directory itself
- `rsync -auL path/to/source path/to/destination` : use archive mode, resolve symlinks, and skip files that are newer on the destination
- `rsync -r --delete rsync://host:path/to/source path/to/destination` : transfer a directory from a remote host running `rsyncd` and delete files on the destination that do not exist on the source
- `rsync -e 'ssh -p port' --info=progress2 host:path/to/source path/to/destination` : transfer a file over SSH using a different port than the default (22) and show global progress

## Categories
network, files

## Links
- More information: https://download.samba.org/pub/rsync/rsync.1
- tldr page: https://tldr.inbrowser.app/pages/common/rsync
