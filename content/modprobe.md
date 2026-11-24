# modprobe

Add or remove modules from the Linux kernel.

## Usage
- `modprobe -n module_name` : simulate loading a module into the kernel
- `sudo modprobe module_name` : load a module into the kernel
- `sudo modprobe -a module_name1 module_name2 ...` : load multiple modules
- `sudo modprobe -r module_name` : remove a module from the kernel
- `sudo modprobe -r --remove-holders module_name` : remove a module and those that depend on it from the kernel
- `sudo modprobe -D module_name` : show a kernel module's dependencies

## See also
- lsmod
- kmod

## Categories
system, development, users

## Links
- More information: https://manned.org/modprobe
- tldr page: https://tldr.inbrowser.app/pages/linux/modprobe
- Wikipedia: https://en.wikipedia.org/wiki/Modprobe
