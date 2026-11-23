# nxc ssh

Pentest and exploit SSH servers.

## Usage
- `nxc ssh 192.168.178.2 -u path/to/usernames.txt -p password` : spray the specified password against a list of usernames on the specified target
- `nxc ssh 192.168.178.2 -u path/to/usernames.txt -p path/to/passwords.txt` : search for valid credentials by trying out every combination in the specified lists of usernames and passwords
- `nxc ssh 192.186.178.2 -u path/to/usernames.txt -p password --key-file path/to/id_rsa` : use the specified private key for authentication, using the supplied password as the key's passphrase
- `nxc ssh 192.168.178.0/24 -u username -p password` : try a combination of username and password on a number of targets
- `nxc ssh 192.168.178.2 -u username -p path/to/passwords.txt --sudo-check` : check for `sudo` privileges on a successful login

## See also
- hydra

## Categories
network, users, files

## Links
- More information: https://www.netexec.wiki/ssh-protocol/password-spraying
- tldr page: https://tldr.inbrowser.app/pages/common/nxc-ssh
