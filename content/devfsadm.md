# devfsadm

Administer `/dev`. Maintains the `/dev` namespace.

## Usage
- `devfsadm -c disk` : scan for new disks
- `devfsadm -C -v` : clean up any dangling `/dev` links and scan for new devices
- `devfsadm -C -v -n` : simulate what would happen but make no modifications

## Notes
A SunOS (Solaris) command: this page is for reference, it does not run on Linux or macOS.

## Categories
sunos

## Links
- More information: https://www.unix.com/man-page/sunos/1m/devfsadm
- tldr page: https://tldr.inbrowser.app/pages/sunos/devfsadm
