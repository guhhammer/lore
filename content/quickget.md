# quickget

Download and prepare materials for building a Quickemu virtual machine. Note: The parameter "edition" is sometimes optional.

## Usage
- `quickget --list|--list-csv|--list-json` : display the list of all supported guest operating systems, versions, and variants in various formats
- `quickget os release edition` : download and create the virtual machine configuration for building a Quickemu virtual machine for an OS
- `quickget windows 11` : download configuration for a Windows 11 VM with VirtIO drivers for Windows
- `quickget macos mojave|catalina|big-sur|monterey|ventura|sonoma` : download a macOS recovery image and create a virtual machine configuration
- `quickget --url os release edition` : show an ISO URL for an operating system
- `quickget --check os release edition` : test if an ISO file is available for an operating system
- `quickget --download os release edition` : download an image without building any VM configuration
- `quickget --create-config os path/to/iso` : create a VM configuration for an OS image (this will move the ISO from the source directory)

## See also
- quickemu

## Categories
network, files, containers

## Links
- More information: https://github.com/quickemu-project/quickemu
- tldr page: https://tldr.inbrowser.app/pages/linux/quickget
