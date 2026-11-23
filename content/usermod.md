# usermod

Modify a user account.

## Usage
- `sudo usermod -l new_username username` : change a username
- `sudo usermod -u user_id username` : change a user ID
- `sudo usermod -s path/to/shell username` : change a user shell
- `sudo usermod -aG group1,group2,... username` : add a user to supplementary groups (mind the lack of whitespace)
- `sudo usermod -rG group1,group2,... username` : remove a user from specific groups
- `sudo usermod -m -d path/to/new_home username` : change a user home directory
- `sudo usermod -L username` : lock an account
- `sudo usermod -U username` : unlock an account

## See also
- users
- useradd
- userdel

## Categories
users

## Links
- More information: https://manned.org/usermod
- tldr page: https://tldr.inbrowser.app/pages/linux/usermod
