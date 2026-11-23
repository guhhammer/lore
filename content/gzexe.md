# gzexe

Compress executable files while keeping them executable. Back up the original file, appending `~` to its name, and create a shell script that uncompresses and executes the binary inside it.

## Usage
- `gzexe path/to/executable` : compress an executable file in-place
- `gzexe -d path/to/compressed_executable` : decompress a compressed executable in-place (i.e. convert the shell script back to an uncompressed binary)

## Categories
files, archives, development

## Links
- More information: https://manned.org/gzexe
- tldr page: https://tldr.inbrowser.app/pages/linux/gzexe
