# kernel-install

Add and remove kernel and initrd images to and from `/boot`.

## Usage
- `kernel-install` : show the current kernel configuration that has been configured or auto-detected
- `kernel-install inspect path/to/kernel_image` : show the configuration for a specific kernel image
- `sudo kernel-install add kernel_version path/to/kernel_image` : add a kernel to the bootloader partition
- `sudo kernel-install add kernel_version path/to/kernel_image path/to/initrd1 path/to/initrd2 ...` : add a kernel and initramfs images to the bootloader partition
- `sudo kernel-install remove kernel_version` : remove a kernel from the bootloader partition

## Categories
system, packages, containers

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/kernel-install.html
- tldr page: https://tldr.inbrowser.app/pages/linux/kernel-install
