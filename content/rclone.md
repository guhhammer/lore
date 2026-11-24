# rclone

Copy, synchronize, or move files and directories to and from many cloud services.

## Usage
- `rclone config` : launch an interactive menu to set up rclone
- `rclone lsf remote_name:path/to/directory` : list contents of a directory on an rclone remote
- `rclone copy path/to/source_file_or_directory remote_name:path/to/directory` : copy a file or directory from the local machine to the remote destination
- `rclone copy -i --max-age 24h remote_name:path/to/directory path/to/local_directory` : copy files changed within the past 24 hours to a remote from the local machine, asking the user to confirm each file
- `rclone sync path/to/file_or_directory remote_name:path/to/directory` : mirror a specific file or directory (Note: Unlike copy, sync removes files from the remote if it does not exist locally)
- `rclone -n delete remote_name:path/to/file_or_directory` : simulate deleting a remote file or directory (remove `--dry-run` from the command to actually delete)
- `rclone mount remote_name:path/to/directory path/to/mount_point` : mount rclone remote (experimental)
- `fusermount -u path/to/mount_point` : unmount rclone remote if `<Ctrl c>` fails (experimental)

## Categories
files, cloud, system

## Links
- More information: https://rclone.org/commands/rclone/
- tldr page: https://tldr.inbrowser.app/pages/common/rclone
- Wikipedia: https://en.wikipedia.org/wiki/Rclone
