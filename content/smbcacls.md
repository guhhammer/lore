# smbcacls

View and manipulate Windows ACLs on SMB shares. Part of the Samba suite.

## Usage
- `smbcacls //server/share path/to/file_or_directory --user domain\\username%password` : display the ACLs for a file or directory on a remote SMB share
- `smbcacls //server/share path/to/file --user domain\\username%password "ACL:DACL"` : set a new ACL for a file on a remote SMB share (replace `"ACL:..."` with a valid Windows ACL specification)
- `smbcacls //server/share path/to/file --user domain\\username%password "RESET" "ACL:DACL"` : remove all existing ACL entries and set a new ACL
- `smbcacls //server/share path/to/file --user username --workgroup workgroup` : specify an alternative workgroup (or domain) and have the program prompt for a password interactively

## Categories
files, users

## Links
- More information: https://www.samba.org/samba/docs/current/man-html/smbcacls.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/smbcacls
