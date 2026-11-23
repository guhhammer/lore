# warp-cli

Connect, disconnect, and switch modes of a connection to Cloudflare's WARP service. WARP is a VPN that encrypts traffic for privacy, security, and speed.

## Usage
- `warp-cli registration new` : register the current device to WARP (must be run before first connection)
- `warp-cli registration show` : display the current registration information
- `warp-cli connect` : connect to WARP
- `warp-cli disconnect` : disconnect from WARP
- `warp-cli status` : display the WARP connection status
- `warp-cli settings list` : display current application settings
- `warp-cli mode warp|doh|warp+doh|dot|warp+dot|proxy|tunnel_only` : switch to a specific mode

## See also
- fastd
- ivpn
- mozillavpn
- mullvad

## Categories
desktop, network, system

## Links
- More information: https://developers.cloudflare.com/warp-client/
- tldr page: https://tldr.inbrowser.app/pages/common/warp-cli
