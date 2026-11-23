# swtpm_ioctl

Utility for sending control commands to swtpm.

## Usage
- `swtpm_ioctl [COMMAND] [<device>]` : the general form, as the manual gives it
- `swtpm_ioctl --tpm-device <device>` : use the given device
- `swtpm_ioctl --unix <path>` : connect to the given UnixIO path
- `swtpm_ioctl -c` : get the capability flags indicating which commands are supported
- `swtpm_ioctl -i` : send  a hardware initialization signal to the swtpm_cuse/swtpm

## Categories
hardware
