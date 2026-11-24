# tailscale up

Connect the client to the Tailscale network. All options described here can be changed later using `tailscale set --option argument`. Use `--option=false` to disable options that don't require arguments. Note: Run `sudo tailscale set --operator $USER` to allow the current user to run these commands.

## Usage
- `tailscale up` : connect to Tailscale
- `tailscale up --advertise-exit-node` : connect and offer the current machine to be an exit node for internet traffic
- `tailscale up --exit-node exit_node_ip` : connect using a specific node for internet traffic
- `tailscale up --shields-up` : connect and block incoming connections to the current node
- `tailscale up --accept-dns=false` : connect and don't accept DNS configuration from the admin panel (defaults to `true`)
- `tailscale up --advertise-routes 10.0.0.0/24,10.0.1.0/24,...` : connect and configure Tailscale as a subnet router
- `tailscale up --accept-routes` : connect and accept subnet routes from Tailscale
- `tailscale up --reset` : reset unspecified settings to their default values and connect

## Categories
users, network

## Links
- More information: https://tailscale.com/kb/1080/cli/#up
- tldr page: https://tldr.inbrowser.app/pages/common/tailscale-up
- Wikipedia: https://en.wikipedia.org/wiki/Tailscale
