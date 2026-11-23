# fastd

VPN daemon. Works on Layer 2 or Layer 3, supports different encryption methods, used by Freifunk.

## Usage
- `fastd -c path/to/fastd.conf` : start `fastd` with a specific configuration file
- `fastd -m tap -M 1400 -c path/to/fastd.conf` : start a Layer 3 VPN with an MTU of 1400, loading the rest of the configuration parameters from a file
- `fastd --verify-config -c path/to/fastd.conf` : validate a configuration file
- `fastd --generate-key` : generate a new keypair
- `fastd --show-key -c path/to/fastd.conf` : show the public key to a private key in a configuration file
- `fastd -v` : display version

## See also
- ivpn
- mozillavpn
- mullvad
- warp-cli

## Categories
files, system

## Links
- More information: https://fastd.readthedocs.io/en/stable/
- tldr page: https://tldr.inbrowser.app/pages/common/fastd
