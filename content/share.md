# share

Make local resource/filesystem available for mounting by remote systems.

## Usage
- `share` : list all currently shared filesystems
- `share -F nfs -o rw /path/to/directory` : share a directory with read/write access
- `share -F nfs -o ro /path/to/directory` : share a directory with read-only access
- `share -F nfs -o rw,root=hostname /path/to/directory` : share a directory with specific options (e.g., allow root access from a specific host)
- `echo "share -F nfs -o rw /path/to/directory" >>/etc/dfs/dfstab` : make sharing persistent by adding entries to `/etc/dfs/dfstab`

## Notes
A SunOS (Solaris) command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, disks, network, sunos

## Links
- More information: https://docs.oracle.com/cd/E36784_01/html/E36825/gntjt.html
- tldr page: https://tldr.inbrowser.app/pages/sunos/share
- Wikipedia: https://en.wikipedia.org/wiki/Share_(command)
