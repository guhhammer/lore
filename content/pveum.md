# pveum

Manage proxmox users.

## Usage
- `pveum u l` : list users
- `pveum u a username@pve` : add a user
- `pveum u a username@pve --email email_address --comment description --password password` : add a user with an email, description, and password
- `pveum pa username@pve` : change user password
- `pveum u d username@pve` : delete a user
- `pveum a m /vms/1000 --user username@pve --role PVEVMUser` : give a user access to a specific VM with specific permissions

## Categories
users

## Links
- More information: https://pve.proxmox.com/pve-docs/pveum.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/pveum
