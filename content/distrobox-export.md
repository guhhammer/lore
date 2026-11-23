# distrobox-export

Export app/service/binary from container to host OS.

## Usage
- `distrobox-export -a package -ef "--foreground"` : export an app from the container to the host (the desktop entry/icon will show up in your host system's application list)
- `distrobox-export -b path/to/binary -ep path/to/binary_on_host` : export a binary from the container to the host
- `distrobox-export -b path/to/binary -ep path/to/export` : export a binary from the container to the host (i.e. `$HOME/.local/bin`)
- `distrobox-export --service package -ef "--allow-newer-config" -S` : export a service from the container to the host (`--sudo` will run the service as root inside the container)
- `distrobox-export -a package -d` : unexport/delete an exported application

## See also
- distrobox

## Categories
network, containers, system

## Links
- More information: https://distrobox.it/usage/distrobox-export/
- tldr page: https://tldr.inbrowser.app/pages/linux/distrobox-export
