# unzipsfx

Create a self-extracting compressed binary file by prepending self-extracting stubs on a Zip file.

## Usage
- `cat $(which unzipsfx) path/to/archive.zip >filename && chmod 755 filename` : create a self-extracting binary file of a Zip archive
- `./path/to/binary` : extract a self-extracting binary in the current directory
- `./path/to/binary -t` : test a self-extracting binary for errors
- `./path/to/binary -c path/to/file` : print content of a file in the self-extracting binary without extraction
- `./path/to/binary -z` : print comments on Zip archive in the self-extracting binary

## Categories
files, archives

## Links
- More information: https://manned.org/unzipsfx
- tldr page: https://tldr.inbrowser.app/pages/linux/unzipsfx
