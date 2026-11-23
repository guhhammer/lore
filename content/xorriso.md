# xorriso

Create, load, manipulate, and write ISO 9660 filesystem images.

## Usage
- `xorriso -outdev path/to/image.iso -map path/to/directory /` : create an ISO image from a directory
- `xorriso -as mkisofs -o path/to/image.iso -V label -J -R path/to/directory` : create an ISO image with a volume label using `mkisofs`-compatible options
- `xorriso -indev path/to/image.iso -ls /` : list files in the root directory of an ISO image
- `xorriso -indev path/to/image.iso -find / -name '*.txt'` : find all files in an ISO image matching a pattern
- `xorriso -osirrox on -indev path/to/image.iso -extract / path/to/output_directory` : extract all files from an ISO image to a directory
- `xorriso -osirrox on -indev path/to/image.iso -extract /path/to/file path/to/output_file` : extract a single file from an ISO image
- `xorriso -dev path/to/image.iso -map path/to/file /path/in/image` : add a file to an existing ISO image
- `xorriso -indev path/to/image.iso|/dev/sr0 -toc` : show the table of contents (sessions and size) of an ISO image or disc

## See also
- mkisofs
- grub-mkrescue

## Categories
files, containers, media

## Links
- More information: https://www.gnu.org/software/xorriso/man_1_xorriso.html
- tldr page: https://tldr.inbrowser.app/pages/common/xorriso
