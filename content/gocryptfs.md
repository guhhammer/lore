# gocryptfs

Encrypted overlay filesystem written in Go.

## Usage
- `gocryptfs -init path/to/cipher_directory` : initialize an encrypted filesystem
- `gocryptfs path/to/cipher_directory path/to/mount_point` : mount an encrypted filesystem
- `gocryptfs --masterkey path/to/cipher_directory path/to/mount_point` : mount with the explicit master key instead of password
- `gocryptfs --passwd path/to/cipher_directory` : change the password
- `gocryptfs --reverse path/to/plain_directory path/to/cipher_directory` : make an encrypted snapshot of a plain directory

## Categories
files, disks

## Links
- More information: https://github.com/rfjakob/gocryptfs#use
- tldr page: https://tldr.inbrowser.app/pages/common/gocryptfs
