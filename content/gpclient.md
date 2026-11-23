# gpclient

Connect to a GlobalProtect VPN on Linux via OpenConnect.

## Usage
- `gpclient connect vpn_gateway_url` : connect to a GlobalProtect VPN using a portal server
- `gpclient disconnect` : disconnect from the currently connected VPN server
- `gpclient launch-gui` : launch the graphical user interface (GUI) for VPN management
- `gpclient connect --fix-openssl vpn_gateway_url` : use OpenSSL workaround to bypass legacy renegotiation errors
- `gpclient connect --ignore-tls-errors vpn_gateway_url` : ignore TLS errors during connection
- `gpclient help command` : display help for any command
- `gpclient --version` : display version

## Categories
network

## Links
- More information: https://github.com/yuezk/GlobalProtect-openconnect
- tldr page: https://tldr.inbrowser.app/pages/linux/gpclient
