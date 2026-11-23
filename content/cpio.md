# cpio

Copy files in and out of archives. Supports the following archive formats: cpio's custom binary, old ASCII, new ASCII, crc, HPUX binary, HPUX old ASCII, old tar, and POSIX.1 tar.

## Usage
- `echo "path/to/file1 path/to/file2 ..." | cpio -o >archive.cpio` : take a list of file names from `stdin` and add them onto an archive (copy-out) in cpio's binary forma
- `find path/to/directory | cpio -ov >archive.cpio` : copy all files and directories in a directory and add them onto an archive (copy-out), in verbose mode
- `cpio <archive.cpio -idv` : pick all files from an archive (copy-in), generating directories where needed, in verbose mode

## Categories
archives, files

## Links
- More information: https://www.gnu.org/software/cpio/manual/cpio.html#Invoking-cpio
- tldr page: https://tldr.inbrowser.app/pages/common/cpio
