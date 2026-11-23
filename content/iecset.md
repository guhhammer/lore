# iecset

Set or dump IEC958 status bits.

## Usage
- `iecset [options] [cmd arg...]` : the general form, as the manual gives it
- `iecset -D device` : specifies the device name of the control to open
- `iecset -c card` : specifies the card index to open
- `iecset -x` : dumps the status in the form of AESx bytes
- `iecset -i` : reads the command sequences from stdin
