# tunelp

Set various parameters for parallel port devices for troubleshooting or for better performance. Part of `util-linux`.

## Usage
- `tunelp -s /dev/lp0` : check the status of a parallel port device
- `tunelp -r /dev/lp0` : reset a given parallel port
- `tunelp -i 5 /dev/lp0` : use a given IRQ for a device, each one representing an interrupt line
- `tunelp -c times -t time_in_centiseconds /dev/lp0` : try a given number of times to output a character to the printer before sleeping for a given time
- `tunelp -a on|off` : enable or disable aborting on error (disabled by default)

## Categories
network, hardware, monitoring

## Links
- More information: https://manned.org/tunelp
- tldr page: https://tldr.inbrowser.app/pages/linux/tunelp
