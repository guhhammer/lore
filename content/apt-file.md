# apt-file

Search for files in `apt` packages, including ones not yet installed.

## Usage
- `sudo apt update` : update the metadata database
- `apt-file find path/to/file` : search for packages that contain the specified file or path
- `apt-file list package` : list the contents of a specific package
- `apt-file find -x regex` : search for packages that match the `regex`

## Categories
files, packages

## Links
- More information: https://manned.org/apt-file
- tldr page: https://tldr.inbrowser.app/pages/linux/apt-file
