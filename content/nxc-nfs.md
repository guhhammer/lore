# nxc nfs

Pentest and exploit NFS servers. Currently supports anonymous mode only.

## Usage
- `nxc nfs 192.168.178.0/24` : detect the version of a remote NFS server
- `nxc nfs 192.168.178.2 --shares` : list the available NFS shares
- `nxc nfs 192.168.178.2 --enum-shares 5` : enumerate the exposed shares recursively to the specified depth
- `nxc nfs 192.168.178.2 --get-file path/to/remote_file path/to/local_file` : download the specified remote file
- `nxc nfs 192.168.178.2 --put-file path/to/local_file path/to/remote_file` : upload the specified local file to the remote share

## Categories
files, network

## Links
- More information: https://www.netexec.wiki/nfs-protocol/enumeration
- tldr page: https://tldr.inbrowser.app/pages/common/nxc-nfs
