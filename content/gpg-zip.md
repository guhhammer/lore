# gpg-zip

Encrypt files and directories in an archive using GPG.

## Usage
- `gpg-zip -c -o archive.gpg path/to/directory` : encrypt a directory into `archive.gpg` using a passphrase
- `gpg-zip -d path/to/archive.gpg` : decrypt `archive.gpg` into a directory of the same name
- `gpg-zip --list-archive path/to/archive.gpg` : list the contents of the encrypted `archive.gpg`

## Categories
security, archives, files

## Links
- More information: https://www.gnupg.org/documentation/manuals/gnupg/gpg_002dzip.html
- tldr page: https://tldr.inbrowser.app/pages/common/gpg-zip
