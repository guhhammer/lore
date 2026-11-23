# pvmove

Move extents from one physical volume to another.

## Usage
- `pvmove position_args [ option_args ] [ position_args ]` : the general form, as the manual gives it
- `pvmove --abort` : abort any pvmove operations in progress
- `pvmove --commandprofile String` : the command profile to use for command configuration
- `pvmove --devices PV` : restricts the devices that are visible and accessible to the command
- `pvmove --driverloaded y|n` : if set to no, the command will not attempt to use device-mapper

## Categories
disks, files, hardware
