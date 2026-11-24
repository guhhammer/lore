# apptainer overlay

Manage EXT3 writable overlay images for Apptainer containers.

## Usage
- `apptainer overlay create -s size path/to/image.sif` : add a writable overlay to an existing SIF image
- `apptainer overlay create -s size path/to/overlay.img` : create a standalone EXT3 writable overlay image
- `apptainer overlay create -s size -S path/to/overlay.img` : create a sparse overlay image
- `apptainer overlay create -f -s size path/to/overlay.img` : create an overlay for use with fakeroot
- `apptainer overlay create --create-dir path/to/directory path/to/overlay.img` : create an overlay with a specific directory in the layout
- `apptainer overlay -h` : display help

## Categories
containers, files, media

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_overlay.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-overlay
- Wikipedia: https://en.wikipedia.org/wiki/Apptainer
