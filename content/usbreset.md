# usbreset

Resets a USB device, as if it was unplugged and plugged in again.

## Usage
- `usbreset` : list the devices with their bus and ids
- `usbreset 1234:abcd` : reset a device by vendor and product id
- `usbreset 001/004` : reset a device by bus and device number

## Notes
Needs root. Whatever uses the device loses its connection for a moment.

## Categories
hardware
