# kioclient

Perform network-transparent file operations using KDE's KIO subsystem. Supports URL schemes such as `file:`, `sftp:`, `smb:`, `fish:`, and `trash:`.

## Usage
- `kioclient exec url` : open a URL with its default KDE handler
- `kioclient cat sftp://user@host/path/to/file` : print the contents of a remote file to `stdout`
- `kioclient ls smb://server/share` : list the contents of a remote directory
- `kioclient cp path/to/source1 path/to/source2 ... path/to/destination` : copy one or more files via KIO
- `kioclient mv path/to/source path/to/destination` : move a file via KIO
- `kioclient rm url` : remove a file via KIO
- `kioclient mkdir url` : create a new directory via KIO
- `kioclient --help` : display help

## Categories
files, network, desktop

## Links
- More information: https://manned.org/kioclient
- tldr page: https://tldr.inbrowser.app/pages/common/kioclient
