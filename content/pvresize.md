# pvresize

Resize physical volume(s).

## Usage
- `pvresize position_args [ option_args ]` : the general form, as the manual gives it
- `pvresize --commandprofile String` : the command profile to use for command configuration
- `pvresize --devices PV` : restricts the devices that are visible and accessible to the command
- `pvresize --driverloaded y|n` : if set to no, the command will not attempt to use device-mapper
- `pvresize --journal String` : record information in the systemd journal

## Categories
disks, hardware, system
