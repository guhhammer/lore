# smbclient

FTP-like client to access SMB/CIFS resources on servers.

## Usage
- `smbclient -L server --no-pass` : list available shares on a server anonymously
- `smbclient //server/share` : connect to a share (will prompt for a password)
- `smbclient -U domain/username //server/share` : connect to a share as a specific user
- `smbclient -U domain/username%password //server/share` : connect to a share as a specific user with inline password
- `smbclient -W domain -U username //server/share` : connect to a share using a specific workgroup
- `smbclient -U domain/username //server/share -D path/to/directory -c 'get filename'` : download a file from a specific directory on a share
- `smbclient -U domain/username //server/share -D path/to/directory -c 'put path/to/local_file'` : upload a file to a specific directory on a share

## Categories
files, network, users

## Links
- More information: https://manned.org/smbclient
- tldr page: https://tldr.inbrowser.app/pages/common/smbclient
