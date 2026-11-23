# vgrename

Rename a volume group.

## Usage
- `vgrename position_args [ option_args ]` : the general form, as the manual gives it
- `vgrename --commandprofile String` : the command profile to use for command configuration
- `vgrename --devices PV` : restricts the devices that are visible and accessible to the command
- `vgrename --driverloaded y|n` : if set to no, the command will not attempt to use device-mapper
- `vgrename --journal String` : record information in the systemd journal

## Categories
disks, files, hardware
