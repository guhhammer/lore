# gnmic

A gNMI client. Manage gNMI network device configuration and view operational data.

## Usage
- `gnmic -a ip:port capabilities` : request device capabilities
- `gnmic -a ip:port -u username -p password capabilities` : provide a username and password to fetch device capabilities
- `gnmic -a ip:port get --path path` : get a snapshot of the device state at a specific path
- `gnmic -a ip:port set --update-path path --update-value value` : update device state at a specific path
- `gnmic -a ip:port sub --path path` : subscribe to target state updates under the subtree at a specific path

## Categories
network, files, hardware

## Links
- More information: https://gnmic.openconfig.net/user_guide/configuration_flags/
- tldr page: https://tldr.inbrowser.app/pages/common/gnmic
