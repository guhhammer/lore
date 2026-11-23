# vgmknodes

Create the special files for volume group devices in /dev.

## Usage
- `vgmknodes [ option_args ] [ position_args ]` : the general form, as the manual gives it
- `vgmknodes --commandprofile String` : the command profile to use for command configuration
- `vgmknodes --devices PV` : restricts the devices that are visible and accessible to the command
- `vgmknodes --driverloaded y|n` : if set to no, the command will not attempt to use device-mapper
- `vgmknodes --journal String` : record information in the systemd journal

## Categories
hardware, disks, system
