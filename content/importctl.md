# importctl

Download, Import, or Export disk images.

## Usage
- `sudo importctl pull-tar url path/to/directory` : download image in tarball format from a url via pull
- `sudo importctl pull-raw https://example.com/source.ext name --class=machine|portable|sysext|confext` : pull or download from a remote source that is either raw or `.qcow2` file, and store it as a raw file
- `importctl import-raw path/to/file.ext name --class=machine|portable|sysext|confext` : import a raw disk image into the image directory that is possibly compressed with `xz`, `gzip`, or `bzip2`
- `importctl export-tar --class=machine|portable|sysext|confext name path/to/file.ext` : export a container image as tarball into current working directory

## Categories
files, network, containers

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/importctl.html
- tldr page: https://tldr.inbrowser.app/pages/linux/importctl
