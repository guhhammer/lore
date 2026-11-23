# vgexport

Unregister volume group(s) from the system.

## Usage
- `vgexport option_args position_args [ option_args ]` : the general form, as the manual gives it
- `vgexport --commandprofile String` : the command profile to use for command configuration
- `vgexport --devices PV` : restricts the devices that are visible and accessible to the command
- `vgexport --driverloaded y|n` : if set to no, the command will not attempt to use device-mapper
- `vgexport --journal String` : record information in the systemd journal

## Categories
disks, hardware, system
