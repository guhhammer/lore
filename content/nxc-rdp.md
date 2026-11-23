# nxc rdp

Pentest and exploit RDP servers.

## Usage
- `nxc rdp 192.168.178.2 -u path/to/usernames.txt -p path/to/passwords.txt` : search for valid credentials by trying out every combination in the specified lists of usernames and passwords
- `nxc rdp 192.168.178.2 -u username -p password --screenshot --screentime 10` : take a screenshot after waiting the for specified number of seconds
- `nxc rdp 192.168.178.2 -u username -p password --screenshot --res 1024x768` : take a screenshot in the specified resolution
- `nxc rdp 192.168.178.2 -u username -p password --nla-screenshot` : take a screenshot of the RDP login prompt if Network Level Authentication is disabled

## Categories
media, users, network

## Links
- More information: https://www.netexec.wiki/rdp-protocol/password-spraying
- tldr page: https://tldr.inbrowser.app/pages/common/nxc-rdp
