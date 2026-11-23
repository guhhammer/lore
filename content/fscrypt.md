# fscrypt

Go tool for managing Linux filesystem encryption.

## Usage
- `sudo fscrypt setup` : prepare the root filesystem for use with `fscrypt`
- `fscrypt setup path/to/directory` : prepare a specific mountpoint for use with `fscrypt`
- `fscrypt encrypt path/to/directory` : enable filesystem encryption for a directory
- `fscrypt unlock path/to/encrypted_directory` : unlock an encrypted directory
- `fscrypt lock path/to/encrypted_directory` : lock an encrypted directory

## Categories
files, disks

## Links
- More information: https://github.com/google/fscrypt#example-usage
- tldr page: https://tldr.inbrowser.app/pages/linux/fscrypt
