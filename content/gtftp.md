# gtftp

This command is an alias of GNU `tftp`.

## Usage
- `tftp server_ip port` : connect to a TFTP server with an interactive shell, specifying its IP address and port
- `tftp server_ip -c command` : connect to a TFTP server and execute a TFTP command
- `tftp server_ip -6 -R port:port` : connect to a TFTP server using IPv6 and force originating port to be in Range
- `mode binary|ascii` : [Interactive] Set the transfer mode to binary or ASCII through the tftp client
- `get file` : [Interactive] Download file from a server through the tftp client
- `put file` : [Interactive] Upload file to a server through the tftp client
- `quit` : [Interactive] Exit the tftp client

## Categories
network, files, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gtftp
