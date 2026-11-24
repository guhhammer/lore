# rdesktop

Remote Desktop Protocol client. It can be used to connect the remote computer using the RDP protocol.

## Usage
- `rdesktop -u username -p password host:port` : connect to a remote computer (default port is 3389)
- `rdesktop -u Administrator -p passwd123 192.168.1.111:3389` : connect to a remote computer as `Administrator` with a specific password
- `rdesktop -u username -p password -f host:port` : connect to a remote computer with full screen (press `<Ctrl Alt Enter>` to exist)
- `rdesktop -u username -p password -g 1366x768 host:port` : use a custom resolution (use the letter `x` between the numbers)
- `rdesktop -u username -p password -d domainname host:port` : connect to a remote computer using domain user
- `rdesktop -u username -p password -a 16 host:port` : use the 16-bit color (speed up)

## Categories
network, users, desktop

## Links
- More information: https://manned.org/rdesktop
- tldr page: https://tldr.inbrowser.app/pages/linux/rdesktop
- Wikipedia: https://en.wikipedia.org/wiki/Rdesktop
