# isadump

Examine ISA registers.

## Usage
- `isadump [-y] [-W|-L] [-k V1,V2...] addrreg datareg [bank [bankreg]] #for I2C-like access` : the general form, as the manual gives it
- `isadump -f [-y] [-W|-L] address [range [bank [bankreg]]] #for flat address space` : another form of the same command
- `isadump -f` : enable flat address space mode
- `isadump -y` : disable interactive mode
- `isadump -W` : perform 16-bit reads
- `isadump -L` : perform 32-bit reads

## Links
- Documentation: https://hwmon.wiki.kernel.org/lm_sensors
