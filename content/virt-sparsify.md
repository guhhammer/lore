# virt-sparsify

Make virtual machine drive images thin-provisioned. Note: Use only for offline machines to avoid data corruption.

## Usage
- `virt-sparsify --compress path/to/image.qcow2 path/to/new_image.qcow2` : create a sparsified compressed image without snapshots from an unsparsified one
- `virt-sparsify --in-place path/to/image.img` : sparsify an image in-place
- `virt-sparsify path/to/image --convert qcow2|raw|vdi|... path/to/new_image` : convert from one image format to another

## Categories
containers, media, files

## Links
- More information: https://manned.org/virt-sparsify
- tldr page: https://tldr.inbrowser.app/pages/linux/virt-sparsify
