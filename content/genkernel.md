# genkernel

Gentoo Linux utility to compile and install kernels.

## Usage
- `sudo genkernel all` : automatically compile and install a generic kernel
- `sudo genkernel bzImage|initramfs|kernel|ramdisk` : build and install the bzImage, initramfs, kernel, or ramdisk only
- `sudo genkernel --menuconfig all` : apply changes to the kernel configuration before compiling and installing
- `sudo genkernel --kernname=custom_name all` : generate a kernel with a custom name
- `sudo genkernel --kerneldir=path/to/directory all` : use a kernel source outside the default directory `/usr/src/linux`

## Categories
system, development, packages

## Links
- More information: https://wiki.gentoo.org/wiki/Genkernel
- tldr page: https://tldr.inbrowser.app/pages/linux/genkernel
