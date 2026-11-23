# gpgtar

Encrypt or sign files into an archive.

## Usage
- `gpgtar [options] filename1 [ filename2, ... ] directory1 [ directory2, ... ]` : the general form, as the manual gives it
- `gpgtar -e` : encrypt  given  files  and directories into an archive
- `gpgtar -d` : extract all files from an encrypted archive
- `gpgtar -s` : make a signed archive from the given files and directories
- `gpgtar -t` : list the contents of the specified archive

## Categories
archives, security
