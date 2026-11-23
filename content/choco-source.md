# choco source

Manage sources for packages with Chocolatey.

## Usage
- `choco source list` : list currently available sources
- `choco source add -n name -s url` : add a new package source
- `choco source add -n name -s url -u username -p password` : add a new package source with credentials
- `choco source add -n name -s url --cert path\to\certificate_file` : add a new package source with a client certificate
- `choco source enable -n name` : enable a package source
- `choco source disable -n name` : disable a package source
- `choco source remove -n name` : remove a package source

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
packages, network, windows

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/source/
- tldr page: https://tldr.inbrowser.app/pages/windows/choco-source
