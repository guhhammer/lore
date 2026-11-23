# vgck

Check the consistency of volume group(s).

## Usage
- `vgck option_args position_args [ option_args ] [ position_args ]` : the general form, as the manual gives it
- `vgck --commandprofile String` : the command profile to use for command configuration
- `vgck --devices PV` : restricts the devices that are visible and accessible to the command
- `vgck --driverloaded y|n` : if set to no, the command will not attempt to use device-mapper
- `vgck --journal String` : record information in the systemd journal

## Categories
disks, hardware, system
