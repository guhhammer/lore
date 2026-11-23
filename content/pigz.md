# pigz

Multithreaded zlib compression utility.

## Usage
- `pigz path/to/file` : compress a file with default options
- `pigz -9 path/to/file` : compress a file using the best compression method
- `pigz -0 -p 4 path/to/file` : compress a file using no compression and 4 processors
- `tar cf - path/to/directory | pigz >path/to/file.tar.gz` : compress a directory using `tar`
- `pigz -d archive.gz` : decompress a file
- `pigz -l archive.tar.gz` : list the contents of an archive

## Categories
files, archives

## Links
- More information: https://manned.org/pigz
- tldr page: https://tldr.inbrowser.app/pages/common/pigz
