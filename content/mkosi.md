# mkosi

Build modern, legacy-free Linux images. Part of `systemd`.

## Usage
- `mkosi summary` : show current build configuration to verify what would be built
- `mkosi build -d fedora|debian|ubuntu|arch|opensuse|...` : build an image with default settings (if no distribution is selected, the distribution of the host system is used)
- `mkosi shell` : build an image and run an interactive shell in a systemd-nspawn container of the image
- `mkosi qemu` : boot an image in a virtual machine using QEMU (only supported for disk images or CPIO images when a kernel is provided)
- `mkosi help` : display help

## Categories
containers, media, development

## Links
- More information: https://manned.org/mkosi
- tldr page: https://tldr.inbrowser.app/pages/linux/mkosi
