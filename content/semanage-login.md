# semanage login

Manage SELinux login mappings between Linux users and SELinux users.

## Usage
- `sudo semanage login -l` : list all login mappings
- `sudo semanage login -a -s selinux_user linux_username` : add a login mapping (map Linux user to SELinux user)
- `sudo semanage login -d linux_username` : delete a login mapping
- `sudo semanage login -m -s selinux_user linux_username` : modify an existing login mapping
- `sudo semanage login -a -s user_u -r s0-s0:c0.c1023 linux_username` : add a login mapping with a specific MLS/MCS range
- `sudo semanage login -l -C` : list only customized login mappings

## See also
- semanage user

## Categories
users

## Links
- More information: https://manned.org/semanage-login
- tldr page: https://tldr.inbrowser.app/pages/linux/semanage-login
