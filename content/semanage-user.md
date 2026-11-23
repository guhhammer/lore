# semanage user

Manage SELinux user mappings.

## Usage
- `sudo semanage user -l` : list all SELinux users
- `sudo semanage user -a -R role_name selinux_user` : add a new SELinux user
- `sudo semanage user -d selinux_user` : delete a SELinux user
- `sudo semanage user -m -R role_name selinux_user` : modify an existing SELinux user's roles
- `sudo semanage user -a -R role_name -L s0 selinux_user` : add a SELinux user with a specific default level
- `sudo semanage user -a -R role_name -r s0-s0:c0.c1023 selinux_user` : add a SELinux user with a specific MLS/MCS range
- `sudo semanage user -l -C` : list only customized SELinux users

## See also
- semanage login

## Categories
users

## Links
- More information: https://manned.org/semanage-user
- tldr page: https://tldr.inbrowser.app/pages/linux/semanage-user
