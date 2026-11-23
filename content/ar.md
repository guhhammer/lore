# ar

Create, modify, and extract from Unix archives. Typically used for static libraries (`.a`) and Debian packages (`.deb`).

## Usage
- `ar x path/to/file.a` : extract all members from an archive
- `ar t path/to/file.ar` : list contents in a specific archive
- `ar r path/to/file.deb path/to/debian-binary path/to/control.tar.gz path/to/data.tar.xz ...` : replace or add specific files to an archive
- `ar s path/to/file.a` : insert an object file index (equivalent to using `ranlib`)
- `ar rs path/to/file.a path/to/file1.o path/to/file2.o ...` : create an archive with specific files and an accompanying object file index

## See also
- tar

## Categories
files, archives, packages

## Links
- More information: https://manned.org/ar
- tldr page: https://tldr.inbrowser.app/pages/common/ar
