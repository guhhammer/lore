# macchanger

Manipulate network interface MAC addresses.

## Usage
- `macchanger -s interface` : view the current and permanent MAC addresses of an interface
- `macchanger -r interface` : set interface to a random MAC
- `macchanger -r -b interface` : set an interface to a random MAC address, and pretend to be a burned-in-address
- `macchanger -m XX:XX:XX:XX:XX:XX interface` : set an interface to a specific MAC address
- `macchanger -l` : print the identifications (the first three bytes of a MAC address) of all known vendors
- `macchanger -p interface` : reset an interface to its permanent hardware MAC address

## Categories
network

## Links
- More information: https://manned.org/macchanger
- tldr page: https://tldr.inbrowser.app/pages/linux/macchanger
