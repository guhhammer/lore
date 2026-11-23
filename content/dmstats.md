# dmstats

Device-mapper statistics management.

## Usage
- `dmsetup stats command [OPTIONS]` : the general form, as the manual gives it
- `dmstats command device_name | --major major --minor minor | -u|--uuid uuid [-v|--verbose]` : another form of the same command
- `dmstats clear device_name [--allprograms|--programid id] [--allregions|--regionid id]` : another form of the same command
- `dmstats delete device_name|--alldevices [--allprograms|--programid id] [--allregions|--regionid id]` : another form of the same command
- `dmstats group [device_name|--alldevices] [--alias name] [--regions regions]` : another form of the same command
- `dmstats --alias name` : specify an alias name for a group
- `dmstats --allprograms` : include regions from all program IDs for list and report operations

## Categories
hardware, monitoring, shell

## Links
- Documentation: https://www.sourceware.org/lvm2
