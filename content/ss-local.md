# ss-local

Run a Shadowsocks client as a SOCKS5 proxy.

## Usage
- `ss-local -s host -p server_port -l local_port -k password -m encrypt_method` : run a Shadowsocks proxy by specifying the host, server port, local port, password, and encryption method
- `ss-local -c path/to/config_file.json` : run a Shadowsocks proxy by specifying the configuration file
- `ss-local --plugin plugin_name --plugin-opts plugin_options` : use a plugin to run the proxy client
- `ss-local --fast-open` : enable TCP fast open

## Categories
network

## Links
- More information: https://manned.org/ss-local
- tldr page: https://tldr.inbrowser.app/pages/common/ss-local
