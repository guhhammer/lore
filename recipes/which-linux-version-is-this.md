# which linux version is this

The distribution, its version, the kernel and the hardware it runs on.

## Steps
- `cat /etc/os-release` : the distribution and its version
- `hostnamectl` : the same, with the kernel and the machine
- `uname -r` : the kernel's version
- `uname -m` : the processor's kind: x86_64, aarch64
- `sw_vers` : macOS's version

## See also
- uname
- hostnamectl
- lsb_release

## Categories
system
