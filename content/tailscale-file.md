# tailscale file

Send files across connected devices on a Tailscale network. It currently does not support sending files to devices owned by other users even on the same Tailscale network.

## Usage
- `tailscale file cp path/to/file hostname|ip:` : send a file to a specific node
- `tailscale file get path/to/directory` : store files that were sent to the current node into a specific directory

## Categories
files, network, hardware

## Links
- More information: https://tailscale.com/kb/1106/taildrop
- tldr page: https://tldr.inbrowser.app/pages/common/tailscale-file
