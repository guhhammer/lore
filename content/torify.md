# torify

Route network traffic through the Tor network. Note: This command is deprecated, use `torsocks` instead. `torify` is now a backwards-compatible wrapper of `torsocks`.

## Usage
- `torify command` : route traffic via Tor
- `torify on|off` : toggle Tor in shell
- `torify --shell` : spawn a Tor-enabled shell
- `torify show` : check for a Tor-enabled shell
- `torify -c config_file command` : specify Tor configuration file
- `torify -P proxy command` : use a specific Tor SOCKS proxy
- `torify command >path/to/output` : redirect output to a file

## Categories
network, shell, files

## Links
- More information: https://manned.org/torify
- tldr page: https://tldr.inbrowser.app/pages/linux/torify
