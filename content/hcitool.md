# hcitool

Monitor, configure connections, and send special commands to Bluetooth devices.

## Usage
- `hcitool scan` : scan for Bluetooth devices
- `hcitool name bdaddr` : output the name of a device, returning its MAC address
- `hcitool info bdaddr` : fetch information about a remote Bluetooth device
- `hcitool lq bdaddr` : check the link quality to a Bluetooth device
- `hcitool tpl bdaddr 0|1` : modify the transmit power level
- `hcitool lp` : display the link policy
- `hcitool auth bdaddr` : request authentication with a specific device
- `hcitool dev` : display local devices

## Categories
hardware, monitoring

## Links
- More information: https://manned.org/hcitool
- tldr page: https://tldr.inbrowser.app/pages/linux/hcitool
