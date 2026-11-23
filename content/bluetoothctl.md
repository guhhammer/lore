# bluetoothctl

Manage Bluetooth devices.

## Usage
- `bluetoothctl` : enter the `bluetoothctl` interactive shell
- `bluetoothctl devices` : list all known devices
- `bluetoothctl power on|off` : power the Bluetooth controller on or off
- `bluetoothctl -t 10 scan on` : scan for available devices for 10 seconds
- `bluetoothctl pair mac_address` : pair with a device
- `bluetoothctl connect|disconnect mac_address` : connect to or disconnect from a paired device
- `bluetoothctl trust mac_address` : allow the device to connect back
- `bluetoothctl remove mac_address` : remove a device

## See also
- bluetui

## Categories
hardware

## Links
- More information: https://manned.org/bluetoothctl
- tldr page: https://tldr.inbrowser.app/pages/linux/bluetoothctl
