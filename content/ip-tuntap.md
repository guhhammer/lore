# ip tuntap

Manage TUN/TAP virtual network interfaces.

## Usage
- `ip tunt` : show all existing TUN/TAP devices
- `sudo ip tunt a d tun0 m t` : create a TUN device with a specific name
- `sudo ip tunt a d tap0 m ta` : create a TAP device with a specific name
- `sudo ip tunt d d tun0|tap0 m tun|tap` : delete a TUN or TAP device
- `sudo ip tunt a d tun0|tap0 m tun|tap u username` : set the owner (UID) of a TUN/TAP device
- `sudo ip tunt a d tun0|tap0 m tun|tap u username g group_name` : set both owner (UID) and group (GID) for a TUN/TAP device

## Categories
hardware, users, network

## Links
- More information: https://baturin.org/docs/iproute2/#ip-tuntap
- tldr page: https://tldr.inbrowser.app/pages/linux/ip-tuntap
