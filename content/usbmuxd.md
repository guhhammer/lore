# usbmuxd

Expose a socket to multiplex connections from and to iOS devices.

## Usage
- `usbmuxd [OPTIONS]` : the general form, as the manual gives it
- `usbmuxd -U` : change to this user after startup (needs USB privileges)
- `usbmuxd -f` : do not daemonize (implies one -v)
- `usbmuxd -n` : disables automatic discovery of devices on hotplug
- `usbmuxd -u` : run in udev operation mode (implies -n and -z)

## Categories
hardware, network

## Links
- Documentation: https://libimobiledevice.org
