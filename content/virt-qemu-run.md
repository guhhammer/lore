# virt-qemu-run

Experimental tool to run a QEMU Guest VM independent of `libvirtd`.

## Usage
- `virt-qemu-run path/to/guest.xml` : run a QEMU virtual machine
- `virt-qemu-run -r path/to/directory path/to/guest.xml` : run a QEMU virtual machine and store the state in a specific directory
- `virt-qemu-run -v path/to/guest.xml` : run a QEMU virtual machine and display verbose information about the startup
- `virt-qemu-run -h` : display help

## Categories
files, text

## Links
- More information: https://libvirt.org/manpages/virt-qemu-run.html
- tldr page: https://tldr.inbrowser.app/pages/linux/virt-qemu-run
