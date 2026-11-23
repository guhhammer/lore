# nxc ftp

Pentest and exploit FTP servers.

## Usage
- `nxc ftp 192.168.178.2 -u path/to/usernames.txt -p path/to/passwords.txt` : search for valid credentials by trying out every combination in the specified lists of usernames and passwords
- `nxc ftp 192.168.178.2 -u path/to/usernames.txt -p path/to/passwords.txt --continue-on-success` : continue searching for valid credentials even after valid credentials have been found
- `nxc ftp 192.168.178.0/24 -u username -p password --ls` : perform directory listings on each FTP server the supplied credentials are valid on
- `nxc ftp 192.168.178.2 -u username -p password --get path/to/file` : download the specified file from the target server
- `nxc ftp 192.168.178.2 -u username -p password --put path/to/local_file path/to/remote_location` : upload the specified file to the target server at the specified location

## Categories
network, files, users

## Links
- More information: https://www.netexec.wiki/ftp-protocol/password-spraying
- tldr page: https://tldr.inbrowser.app/pages/common/nxc-ftp
