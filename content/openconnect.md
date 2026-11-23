# openconnect

A VPN client, for Cisco AnyConnect VPNs and others.

## Usage
- `sudo openconnect vpn.example.org` : connect to a server
- `sudo openconnect -b vpn.example.org` : connect to a server, forking into the background
- `sudo killall -2 openconnect` : terminate the connection that is running in the background
- `sudo openconnect -u username vpn.example.org` : specify the user to log in as
- `sudo openconnect --protocol nc|pulse|gp|f5|fortinet|array vpn.example.org` : specify the connection protocol
- `sudo openconnect --config path/to/config_file vpn.example.org` : connect to a server, reading options from a configuration file
- `sudo openconnect -c path/to/certificate.pem vpn.example.org` : connect to a server and authenticate with a specific SSL client certificate

## Categories
users, files, security

## Links
- More information: https://www.infradead.org/openconnect/manual.html
- tldr page: https://tldr.inbrowser.app/pages/common/openconnect
