# bootc switch

Target a new container image reference to boot.

## Usage
- `sudo bootc switch image` : change the base OS to a new container image from a registry
- `sudo bootc switch --transport containers-storage image` : change the base OS to a new container image from the local image storage of the root user
- `sudo bootc switch --transport oci-archive path/to/image.tar.gz` : change the base OS to a new container image stored in a tarball

## Categories
containers, media, users

## Links
- More information: https://manned.org/bootc-switch
- tldr page: https://tldr.inbrowser.app/pages/linux/bootc-switch
