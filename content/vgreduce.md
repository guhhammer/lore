# vgreduce

Remove physical volume(s) from a volume group.

## Usage
- `vgreduce --commandprofile String` : the command profile to use for command configuration
- `vgreduce --devices PV` : restricts the devices that are visible and accessible to the command
- `vgreduce --driverloaded y|n` : if set to no, the command will not attempt to use device-mapper
- `vgreduce --journal String` : record information in the systemd journal

## Categories
disks, hardware, system
