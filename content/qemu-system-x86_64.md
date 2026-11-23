# qemu-system-x86_64

Emulate the `x86_64` architecture.

## Usage
- `qemu-system-x86_64 -hda image_name.img -m 4096` : boot from an image emulating the `x86_64` architecture
- `qemu-system-x86_64 -hda image_name.img -m 4096 -cdrom os_image.iso -boot d` : boot a QEMU instance from a live ISO image
- `qemu-system-x86_64 -hda /dev/storage_device -m 4096` : boot from a physical device (e.g. from USB to test a bootable medium)
- `qemu-system-x86_64 -hda image_name.img -m 4096 -nographic` : do not launch a VNC server
- `<Ctrl a><x>` : exit non-graphical QEMU
- `qemu-system-x86_64 -M help` : list the supported machine types

## Categories
system

## Links
- More information: https://www.qemu.org/docs/master/system/target-i386.html
- tldr page: https://tldr.inbrowser.app/pages/common/qemu-system-x86_64
