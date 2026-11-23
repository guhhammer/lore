# rsync-ssl

A wrapper that runs rsync over a TLS-encrypted connection to an rsync daemon.

## Usage
- `rsync-ssl -av host::module/ local/` : copy a module from a daemon into a folder, over TLS
- `rsync-ssl --type=openssl -av host::module/ local/` : choose the TLS helper program explicitly
- `rsync-ssl -av local/ host::module/` : send a folder to the daemon

## Categories
security, system, network

## Links
- Documentation: https://download.samba.org/pub/rsync/rsync-ssl.1
