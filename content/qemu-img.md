# qemu-img

Create and manipulate Quick Emulator Virtual HDD images.

## Usage
- `qemu-img create path/to/image_file.img gigabytesG` : create disk image with a specific size (in gigabytes)
- `qemu-img info path/to/image_file.img` : show information about a disk image
- `qemu-img resize path/to/image_file.img gigabytesG` : increase or decrease image size
- `qemu-img map path/to/image_file.img` : dump the allocation state of every sector of the specified disk image
- `qemu-img convert -f vmdk -O qcow2 -p path/to/image_file.vmdk path/to/image_file.qcow2` : convert a VMware `.vmdk` disk image to a KVM `.qcow2` disk image and display progress
- `qemu-img snapshot -c snapshot_tag_name path/to/image_file.qcow2` : create an internal snapshot of a KVM `.qcow2` disk image
- `qemu-img snapshot -a snapshot_tag_name path/to/image_file.qcow2` : apply an internal snapshot to a KVM `.qcow2` disk image

## Categories
containers, media, files

## Links
- More information: https://qemu.readthedocs.io/en/master/tools/qemu-img.html
- tldr page: https://tldr.inbrowser.app/pages/common/qemu-img
