# pdbedit

Edit the Samba user database. For simple user add/remove/password, you can also use `smbpasswd`.

## Usage
- `sudo pdbedit -L` : list all Samba users
- `sudo pdbedit -L -v` : list all Samba users and their settings
- `sudo pdbedit -u username -a` : add an existing Unix user to Samba (will prompt for password)
- `sudo pdbedit -u username -x` : remove a Samba user
- `sudo pdbedit -u username -z` : reset a Samba user's failed password counter

## Categories
users, databases

## Links
- More information: https://manned.org/pdbedit
- tldr page: https://tldr.inbrowser.app/pages/linux/pdbedit
