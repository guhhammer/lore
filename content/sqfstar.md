# sqfstar

Create a squashfs filesystem from a `.tar` archive.

## Usage
- `sqfstar <archive.tar filesystem.squashfs` : create a squashfs filesystem (compressed using `gzip` by default) from an uncompressed `.tar` archive
- `zcat archive.tar.gz | sqfstar -comp gzip|lzo|lz4|xz|zstd|lzma filesystem.squashfs` : create a squashfs filesystem from a `.tar` archive compressed with `gzip`, and [comp]ress the filesystem using a specific algorithm
- `xzcat archive.tar.xz | sqfstar filesystem.squashfs file1 file2 ...` : create a squashfs filesystem from a `.tar` archive compressed with `xz`, excluding some of the files
- `zstdcat archive.tar.zst | sqfstar filesystem.squashfs "*.gz"` : create a squashfs filesystem from a `.tar` archive compressed with `zstd`, excluding files ending with `.gz`
- `lz4cat archive.tar.lz4 | sqfstar filesystem.squashfs -regex "regex"` : create a squashfs filesystem from a `.tar` archive compressed with `lz4`, excluding files matching a `regex`

## Categories
archives, disks, text

## Links
- More information: https://manned.org/sqfstar
- tldr page: https://tldr.inbrowser.app/pages/linux/sqfstar
