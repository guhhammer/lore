# usbip

Use USB devices remotely.

## Usage
- `usbip list -l` : list all local USB devices and their bus ID's
- `systemctl start usbipd` : start a `usbip` daemon on the server
- `sudo usbip bind -b bus_id` : bind a USB device to `usbip` on the server
- `sudo modprobe vhci-hcd` : load the kernel module required by `usbip` on the client
- `sudo usbip attach -r ip_address -b bus_id` : attach to the `usbip` device on the client (bus ID is the same as on the server)
- `usbip port` : list attached devices
- `sudo usbip detach -p port` : detach from a device
- `usbip unbind -b bus_id` : unbind a device

## Categories
hardware, network, users

## Links
- More information: https://manned.org/usbip
- tldr page: https://tldr.inbrowser.app/pages/linux/usbip
