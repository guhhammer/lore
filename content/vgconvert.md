# vgconvert

Change volume group metadata format.

## Usage
- `vgconvert position_args [ option_args ]` : the general form, as the manual gives it
- `vgconvert --commandprofile String` : the command profile to use for command configuration
- `vgconvert --devices PV` : restricts the devices that are visible and accessible to the command
- `vgconvert --driverloaded y|n` : if set to no, the command will not attempt to use device-mapper
- `vgconvert --journal String` : record information in the systemd journal

## Categories
disks, hardware, system
