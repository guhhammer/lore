# dbclient

Lightweight Dropbear Secure Shell client.

## Usage
- `dbclient user@host` : connect to a remote host
- `dbclient user@host -p 2222` : connect to a remote host on port 2222
- `dbclient -i path/to/key_file user@host` : connect to a remote host using a specific identity key in dropbear format
- `dbclient user@host -t command argument1 argument2 ...` : run a command on the remote host with a tty allocation allowing interaction with the remote command
- `dbclient -A user@host` : connect and forward Agent connections to remote host

## Categories
network, users, shell

## Links
- More information: https://manned.org/dbclient
- tldr page: https://tldr.inbrowser.app/pages/linux/dbclient
