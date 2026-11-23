# vgmerge

Merge volume groups.

## Usage
- `vgmerge position_args [ option_args ]` : the general form, as the manual gives it
- `vgmerge --commandprofile String` : the command profile to use for command configuration
- `vgmerge --devices PV` : restricts the devices that are visible and accessible to the command
- `vgmerge --driverloaded y|n` : if set to no, the command will not attempt to use device-mapper
- `vgmerge --journal String` : record information in the systemd journal

## Categories
disks, hardware, system
