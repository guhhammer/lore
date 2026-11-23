# openvpn

OpenVPN client and daemon binary.

## Usage
- `sudo openvpn path/to/client.conf` : connect to server using a configuration file
- `sudo openvpn --remote alice.example.com --dev tun1 --ifconfig 10.4.0.1 10.4.0.2` : try to set up an insecure peer-to-peer tunnel on bob.example.com host
- `sudo openvpn --remote bob.example.com --dev tun1 --ifconfig 10.4.0.2 10.4.0.1` : connect to the awaiting bob.example.com host without encryption
- `openvpn --genkey secret path/to/key` : create a cryptographic key and save it to file
- `sudo openvpn --remote alice.example.com --dev tun1 --ifconfig 10.4.0.1 10.4.0.2 --secret path/to/key` : try to set up a peer-to-peer tunnel on bob.example.com host with a static key
- `sudo openvpn --remote bob.example.com --dev tun1 --ifconfig 10.4.0.2 10.4.0.1 --secret path/to/key` : connect to the awaiting bob.example.com host with the same static key as on bob.example.com

## Categories
files, network, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/openvpn
