# isaset

Set ISA registers.

## Usage
- `isaset [-y] [-W|-L] addrreg datareg address value [mask] #for I2C-like access` : the general form, as the manual gives it
- `isaset -f [-y] [-W|-L] address value [mask] #for flat address space` : another form of the same command
- `isaset -f` : enable flat address space mode
- `isaset -y` : disable  interactive  mode
- `isaset -W` : perform a 16-bit write
- `isaset -L` : perform a 32-bit write

## Links
- Documentation: https://hwmon.wiki.kernel.org/lm_sensors
