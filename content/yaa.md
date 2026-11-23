# yaa

Create and manipulate YAA archives.

## Usage
- `yaa archive -d path/to/directory -o path/to/output_file.yaa` : create an archive from a directory
- `yaa archive -i path/to/file -o path/to/output_file.yaa` : create an archive from a file
- `yaa extract -i path/to/archive_file.yaa` : extract an archive to the current directory
- `yaa list -i path/to/archive_file.yaa` : list the contents of an archive
- `yaa archive -a algorithm -d path/to/directory -o path/to/output_file.yaa` : create an archive with a specific compression algorithm
- `yaa archive -b 8m -d path/to/directory -o path/to/output_file.yaa` : create an archive with an 8 MB block size

## Categories
files, archives, macos

## Links
- More information: https://www.manpagez.com/man/1/yaa/
- tldr page: https://tldr.inbrowser.app/pages/osx/yaa
