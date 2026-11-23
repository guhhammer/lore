# tailscale

A private WireGuard network service. Some subcommands such as `up` have their own usage documentation.

## Usage
- `sudo tailscale set --operator $USER` : allow the current user to operate on the Tailscale daemon
- `tailscale up|down|status` : handle connection to Tailscale (connect, disconnect, check status)
- `tailscale set --advertise-exit-node` : offer the current machine to be an exit node for internet traffic
- `tailscale set --exit-node ip_address|hostname` : use a specific exit node for internet traffic
- `tailscale ping ip_address|hostname` : ping a peer node at the Tailscale layer and display which route it took for each response
- `tailscale netcheck` : analyze the local network conditions and display the result
- `tailscale web` : start a web server for controlling the Tailscale daemon
- `tailscale bugreport` : display a shareable identifier to help diagnose issues

## Categories
network, system, desktop

## Links
- More information: https://tailscale.com/kb/1080/cli
- tldr page: https://tldr.inbrowser.app/pages/common/tailscale
