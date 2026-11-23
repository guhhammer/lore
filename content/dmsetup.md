# dmsetup

Low level logical volume management.

## Usage
- `dmsetup clear device_name` : the general form, as the manual gives it
- `dmsetup create --concise [concise_device_specification]` : another form of the same command
- `dmsetup deps [-o options] [device_name...]` : another form of the same command
- `dmsetup help [-c|-C|--columns]` : another form of the same command
- `dmsetup info [device_name...]` : another form of the same command
- `dmsetup --addnodeoncreate` : ensure /dev/mapper node exists after dmsetup create
- `dmsetup --count count` : specify the number of times to repeat a report

## Categories
disks

## Links
- Documentation: https://www.sourceware.org/lvm2
