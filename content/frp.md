# frp

Fast Reverse Proxy: quickly set up network tunnels to expose certain services to the Internet or other external networks. Note: `frp` isn't a command, but a set of commands.

## Usage
- `frpc` : start the service, using the default configuration file (assumed to be `frps.ini` in the current directory)
- `frpc -c path/to/frpc.toml` : start the service, using the newer TOML configuration file
- `frps` : start the service, using the default configuration file (assumed to be `frps.ini` in the current directory)
- `frps -c path/to/frps.toml` : start the service, using the newer TOML configuration file

## Categories
files, network, system

## Links
- More information: https://github.com/fatedier/frp#example-usage
- tldr page: https://tldr.inbrowser.app/pages/common/frp
