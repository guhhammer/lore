# smbget

`wget`-like utility for downloading files from SMB servers.

## Usage
- `smbget smb://server/share/file` : download a file from a server
- `smbget --recursive smb://server/share` : download a share or directory recursively
- `smbget smb://server/share/file -U username%password` : connect with a username and password
- `smbget smb://server/share/file -e` : require encrypted transfers

## Categories
files, network

## Links
- More information: https://www.samba.org/samba/docs/current/man-html/smbget.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/smbget
