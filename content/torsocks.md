# torsocks

Route the traffic of any application through the Tor network. Note: `torsocks` will assume that it should connect to the Tor SOCKS proxy running at 127.0.0.1:9050 being the defaults of the Tor daemon.

## Usage
- `torsocks command` : run a command using Tor
- `. torsocks on|off` : enable or disable Tor in this shell
- `torsocks --shell` : spawn a new Tor enabled shell
- `torsocks show` : check if current shell is Tor enabled (`$LD_PRELOAD` value will be empty if disabled)
- `torsocks -i curl https://check.torproject.org/api/ip` : isolate traffic through a different Tor circuit, improving anonymity
- `torsocks -a ip_address -P port command` : connect to a Tor proxy running on a specific address and port

## Categories
network, shell, system

## Links
- More information: https://manned.org/torsocks
- tldr page: https://tldr.inbrowser.app/pages/linux/torsocks
