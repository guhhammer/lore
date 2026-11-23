# l2ping

Send an L2CAP echo request and receive an answer.

## Usage
- `sudo l2ping mac_address` : ping a Bluetooth device
- `sudo l2ping -r mac_address` : reverse ping a Bluetooth device
- `sudo l2ping -i hci0 mac_address` : ping a Bluetooth device from a specified interface
- `sudo l2ping -s byte_count mac_address` : ping Bluetooth device with a specified sized data package
- `sudo l2ping -f mac_address` : ping flood a Bluetooth device
- `sudo l2ping -c amount mac_address` : ping a Bluetooth device a specified amount of times
- `sudo l2ping -d seconds mac_address` : ping a Bluetooth device with a specified delay between requests

## Categories
hardware, network, users

## Links
- More information: https://manned.org/l2ping
- tldr page: https://tldr.inbrowser.app/pages/common/l2ping
