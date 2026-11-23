# dcfldd

Enhanced version of dd for forensics and security.

## Usage
- `dcfldd if=/dev/disk_device of=file.img hash=sha256 hashlog=file.hash` : copy a disk to a raw image file and hash the image using SHA256
- `dcfldd if=/dev/disk_device of=file.img hash=sha512|sha384|sha256|sha1|md5 hashlog=file.hash hashwindow=1G` : copy a disk to a raw image file, hashing each 1 GB chunk

## Categories
files, security, containers

## Links
- More information: https://manned.org/dcfldd
- tldr page: https://tldr.inbrowser.app/pages/common/dcfldd
