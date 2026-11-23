# sftp

Secure File Transfer Program. Interactive program to copy files between hosts over SSH. For non-interactive file transfers, see `scp` or `rsync`.

## Usage
- `sftp remote_user@remote_host` : connect to a remote server and enter an interactive command mode
- `sftp -P remote_port remote_user@remote_host` : connect using an alternate port
- `sftp host` : connect using a predefined host (in `~/.ssh/config`)
- `get path/to/remote_file` : [Interactive] Transfer remote file to the local system
- `put path/to/local_file` : [Interactive] Transfer local file to the remote system
- `get -R path/to/remote_directory` : [Interactive] Transfer remote directory to the local system recursively (works with `put` too)
- `lls` : [Interactive] Get list of files on local machine
- `ls` : [Interactive] Get a list of files on the remote machine

## Categories
files, network, security

## Links
- More information: https://manned.org/sftp
- tldr page: https://tldr.inbrowser.app/pages/common/sftp
