# cec-client

Manage serial bus CEC connections.

## Usage
- `cec-client -l` : list all CEC adapters
- `sudo cec-client` : start an interactive CEC session
- `sudo cec-client -o name` : set the On-Screen Display name
- `echo on 0 | sudo cec-client -s` : send a single command
- `standby 0` : [Interactive] Set a device to standby
- `on 0` : [Interactive] Turn a device on

## See also
- cec-ctl

## Categories
users

## Links
- More information: https://manned.org/cec-client
- tldr page: https://tldr.inbrowser.app/pages/linux/cec-client
