# sshpass

An SSH password provider. It works by creating a TTY, feeding the password into it, and then redirecting `stdin` to the SSH session.

## Usage
- `sshpass -d 0 ssh user@hostname` : connect to a remote server using a password supplied on a file descriptor (in this case, `stdin`)
- `sshpass -p password ssh -o StrictHostKeyChecking=no user@hostname` : connect to a remote server with the password supplied as an option, and automatically accept unknown SSH keys
- `sshpass -f path/to/file ssh -o StrictHostKeyChecking=no user@hostname "command"` : connect to a remote server using the first line of a file as the password, automatically accept unknown SSH keys, and launch a command

## Categories
network, users, files

## Links
- More information: https://manned.org/sshpass
- tldr page: https://tldr.inbrowser.app/pages/common/sshpass
