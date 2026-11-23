# portablectl

A systemd utility for managing and deploying portable service images on Linux systems.

## Usage
- `portablectl list` : list available portable service images discovered in the portable image search paths
- `portablectl attach path/to/image` : attach a portable service image to the host system
- `portablectl detach path/to/image|image_name` : detach a portable service image from the host system
- `portablectl inspect path/to/image` : display details and metadata about a specified portable service image
- `portablectl is-attached path/to/image|image_name` : check if a portable service image is attached to the host system

## Categories
containers, media, system

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/portablectl.html
- tldr page: https://tldr.inbrowser.app/pages/linux/portablectl
