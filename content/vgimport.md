# vgimport

Register exported volume group with system.

## Usage
- `vgimport option_args position_args [ option_args ]` : the general form, as the manual gives it
- `vgimport --commandprofile String` : the command profile to use for command configuration
- `vgimport --devices PV` : restricts the devices that are visible and accessible to the command
- `vgimport --driverloaded y|n` : if set to no, the command will not attempt to use device-mapper
- `vgimport --journal String` : record information in the systemd journal

## Categories
disks, hardware, system
