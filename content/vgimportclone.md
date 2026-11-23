# vgimportclone

Import a VG from cloned PVs.

## Usage
- `vgimportclone position_args [ option_args ]` : the general form, as the manual gives it
- `vgimportclone --commandprofile String` : the command profile to use for command configuration
- `vgimportclone --devices PV` : restricts the devices that are visible and accessible to the command
- `vgimportclone --driverloaded y|n` : if set to no, the command will not attempt to use device-mapper
- `vgimportclone --importdevices` : add devices to the devices file

## Categories
hardware
