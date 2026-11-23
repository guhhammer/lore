# ldattach

Attach a line discipline to a serial line.

## Usage
- `ldattach [-1278denoVh] [-i iflag] [-s speed] ldisc device` : the general form, as the manual gives it
- `ldattach -1` : set the number of stop bits of the serial line to one
- `ldattach -2` : set the number of stop bits of the serial line to two
- `ldattach -7` : set the character size of the serial line to 7 bits
- `ldattach -8` : set the character size of the serial line to 8 bits

## Categories
text

## Links
- Documentation: https://www.kernel.org/pub/linux/utils/util-linux/
