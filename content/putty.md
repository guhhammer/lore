# putty

SSH, Telnet, and Rlogin client for connecting to remote servers. Note: On Linux, the native `ssh` client is often more convenient. PuTTY is more commonly used on Windows.

## Usage
- `putty -ssh username@hostname_or_ip` : connect to a remote host via SSH
- `putty -ssh username@hostname_or_ip -P port` : connect to a remote host on a specific Port
- `putty -load session_name` : load a saved session
- `putty -ssh username@hostname_or_ip -i path/to/private_key.ppk` : connect with a private key for authentication
- `putty -telnet hostname_or_ip` : connect via Telnet
- `putty -ssh username@hostname_or_ip -X` : enable X11 forwarding
- `putty -ssh username@hostname_or_ip -L local_port:destination_host:destination_port` : set up Local port forwarding
- `putty -help` : display help

## Categories
network

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/putty
