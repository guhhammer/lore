# tart

Build, run, and manage macOS and Linux virtual machines (VMs) on Apple Silicon.

## Usage
- `tart pull acme.io/org/name:tag` : pull a remote VM image
- `tart clone source-vm vm-name` : clone a VM from a local or remote image source
- `tart create --from-ipsw latest|path/to/file.ipsw vm-name` : create a new Mac VM from a specific ipsw file
- `tart run vm-name` : run an existing VM
- `tart run --dir path/to/directory:/path/to/local_directory vm-name` : run an existing VM with a specific mounted directory
- `tart list` : list VMs
- `tart ip vm-name` : get IP address of a running VM
- `tart set vm-name --display 640x400` : change a VM's display resolution

## Categories
files, development, network, macos

## Links
- More information: https://github.com/cirruslabs/tart
- tldr page: https://tldr.inbrowser.app/pages/osx/tart
