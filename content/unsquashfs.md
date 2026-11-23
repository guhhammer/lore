# unsquashfs

Uncompress, extract, and list files in squashfs filesystems.

## Usage
- `unsquashfs filesystem.squashfs` : extract a squashfs filesystem to `squashfs-root` in the current working directory
- `unsquashfs -d path/to/directory filesystem.squashfs` : extract a squashfs filesystem to the specified directory
- `unsquashfs -i filesystem.squashfs` : display the names of files as they are extracted
- `unsquashfs -li filesystem.squashfs` : display the names of files and their attributes as they are extracted
- `unsquashfs -l filesystem.squashfs` : list files inside the squashfs filesystem (without extracting)
- `unsquashfs -ll filesystem.squashfs` : list files and their attributes inside the squashfs filesystem (without extracting)

## Categories
disks, archives, files

## Links
- More information: https://manned.org/unsquashfs
- tldr page: https://tldr.inbrowser.app/pages/linux/unsquashfs
