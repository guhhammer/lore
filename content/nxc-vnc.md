# nxc vnc

Pentest and exploit VNC servers.

## Usage
- `nxc vnc 192.168.178.2 -u path/to/usernames.txt -p path/to/passwords.txt` : search for valid credentials by trying out every combination in the specified lists of usernames and passwords
- `nxc vnc 192.168.178.2 -u path/to/usernames.txt -p path/to/passwords.txt --vnc-sleep 10` : avoid rate limiting through VNC-sleep
- `nxc vnc 192.168.178.2 -u username -p password --screenshot --screentime 10` : take a screenshot on the remote system after waiting the specified amount of time

## Categories
files

## Links
- More information: https://www.netexec.wiki/getting-started/selecting-and-using-a-protocol
- tldr page: https://tldr.inbrowser.app/pages/common/nxc-vnc
