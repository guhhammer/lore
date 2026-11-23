# getty

Alternative Linux getty.

## Usage
- `agetty [options] port [baud_rate...] [term]` : the general form, as the manual gives it
- `getty -8` : assume that the tty is 8-bit clean, hence disable parity detection
- `getty -h` : enable hardware (RTS/CTS) flow control
- `getty -H` : write the specified fakehost into the utmp file
- `getty -J` : do not clear the screen before prompting for the login name

## Links
- Documentation: https://systemd.io/CREDENTIALS/
