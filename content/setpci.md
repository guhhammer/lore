# setpci

Reads and writes the configuration registers of PCI devices.

## Usage
- `setpci -s 00:1f.2 COMMAND` : read one register of a device
- `setpci -L` : list the register names it knows

## Notes
Writing the wrong value to a device can freeze the system. Needs root.

## Categories
hardware
