# vgsplit

Move physical volumes into a new or existing volume group.

## Usage
- `vgsplit option_args position_args [ option_args ]` : the general form, as the manual gives it
- `vgsplit --commandprofile String` : the command profile to use for command configuration
- `vgsplit --devices PV` : restricts the devices that are visible and accessible to the command
- `vgsplit --driverloaded y|n` : if set to no, the command will not attempt to use device-mapper
- `vgsplit --journal String` : record information in the systemd journal

## Categories
disks, files, hardware
