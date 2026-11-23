# bootc

Boot and upgrade via container images. Manage transactional, in-place operating system updates using OCI/Docker container images.

## Usage
- `bootc status` : show deployments in the order they will appear in the bootloader
- `bootc upgrade --check` : check if any updates are available
- `bootc upgrade --apply` : prepare a new update and reboot into it
- `bootc switch image` : change OS base to new container image
- `bootc rollback` : reboot into the previous ostree deployment
- `bootc edit` : apply transactional changes to the system configuration

## Categories
containers, media, packages

## Links
- More information: https://manned.org/bootc
- tldr page: https://tldr.inbrowser.app/pages/linux/bootc
