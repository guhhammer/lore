# pax

Archiving and copying utility.

## Usage
- `pax -f archive.tar` : list the contents of an archive
- `pax -zf archive.tar.gz` : list the contents of a `gzip` archive
- `pax -wf target.tar path/to/file1 path/to/file2 ...` : create an archive from files
- `pax -w path/to/file1 path/to/file2 ... >target.tar` : create an archive from files, using output redirection
- `pax -rf source.tar` : extract an archive into the current directory
- `pax -rw path/to/file1 path/to/directory1 path/to/directory2 ... target/` : copy to a directory, while keeping the original metadata; `target/` must exist

## Categories
archives, files

## Links
- More information: https://manned.org/pax.1p
- tldr page: https://tldr.inbrowser.app/pages/common/pax
