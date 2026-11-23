# smbpasswd

Add/remove a Samba user or change its password. Samba users must have an existing local Unix account.

## Usage
- `smbpasswd` : change the current user's SMB password
- `sudo smbpasswd -a username` : add a specified user to Samba and set password (user should already exist in system)
- `sudo smbpasswd username` : modify an existing Samba user's password
- `sudo smbpasswd -x username` : delete a Samba user (use `pdbedit` instead if the Unix account has been deleted)

## Categories
users

## Links
- More information: https://manned.org/smbpasswd.8
- tldr page: https://tldr.inbrowser.app/pages/linux/smbpasswd
