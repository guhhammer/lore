# enosys

Utility to make syscalls fail with ENOSYS.

## Usage
- `enosys [--syscall|-s syscall] command` : the general form, as the manual gives it
- `enosys -s` : syscall to block with ENOSYS. Can be specified multiple times
- `enosys -i` : ioctl to block with ENOTTY. Can be specified multiple times
- `enosys -l` : list syscalls known to enosys
- `enosys -m` : list ioctls known to enosys

## Links
- Documentation: https://www.kernel.org/pub/linux/utils/util-linux/
