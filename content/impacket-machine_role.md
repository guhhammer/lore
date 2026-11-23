# impacket-machine_role

This command is an alias of `machine_role.py`.

## Usage
- `machine_role.py domain/username:password@target` : determine the role of a machine using a username and password
- `machine_role.py -hashes LM_Hash:NT_Hash domain/username@target` : determine the role using pass-the-hash authentication
- `machine_role.py -no-pass domain/username@target` : determine the role without prompting for a password (e.g. using an existing session)
- `machine_role.py -k domain/username@target` : determine the role using Kerberos authentication
- `machine_role.py -dc-ip ip_address domain/username:password@target` : specify the domain controller's IP address

## Categories
users, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/impacket-machine_role
