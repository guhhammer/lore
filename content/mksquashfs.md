# mksquashfs

Create or append files and directories to squashfs filesystems.

## Usage
- `mksquashfs path/to/file_or_directory1 path/to/file_or_directory2 ... filesystem.squashfs` : create or append files and directories to a squashfs filesystem (compressed using `gzip` by default)
- `mksquashfs path/to/file_or_directory1 path/to/file_or_directory2 ... filesystem.squashfs -comp gzip|lzo|lz4|xz|zstd|lzma` : create or append files and directories to a squashfs filesystem, using a specific [comp]ression algorithm
- `mksquashfs path/to/file_or_directory1 path/to/file_or_directory2 ... filesystem.squashfs -e file|directory1 file|directory2 ...` : create or append files and directories to a squashfs filesystem, excluding some of them
- `mksquashfs path/to/file_or_directory1 path/to/file_or_directory2 ... filesystem.squashfs -wildcards -e "*.gz"` : create or append files and directories to a squashfs filesystem, excluding those ending with `gzip`
- `mksquashfs path/to/file_or_directory1 path/to/file_or_directory2 ... filesystem.squashfs -regex -e "regex"` : create or append files and directories to a squashfs filesystem, excluding those matching a `regex`

## Categories
files, disks, archives

## Links
- More information: https://manned.org/mksquashfs
- tldr page: https://tldr.inbrowser.app/pages/linux/mksquashfs
