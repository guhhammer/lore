# wishlist

SSH directory and multiplexer. Acts as a single entry point for connecting to SSH servers or Wish applications.

## Usage
- `wishlist` : explore SSH servers listed in your `~/.ssh/config` file (local mode)
- `wishlist s` : start Wishlist in server mode to provide remote access
- `wishlist -c path/to/config.yaml` : use a custom configuration file
- `wishlist --zeroconf.enabled` : discover SSH endpoints using Zeroconf (mDNS/Bonjour)
- `wishlist --srv.domain example.com` : discover SSH nodes from DNS SRV records
- `wishlist --tailscale.net=tailnet_name --tailscale.key=tskey-api-abc123` : discover SSH nodes from a Tailscale tailnet

## Categories
network, files

## Links
- More information: https://github.com/charmbracelet/wishlist
- tldr page: https://tldr.inbrowser.app/pages/common/wishlist
