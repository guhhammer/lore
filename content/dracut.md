# dracut

Generate initramfs images to boot the Linux kernel. Dracut uses options from configuration files in `/etc/dracut.conf`, `/etc/dracut.conf.d/*.conf`, and `/usr/lib/dracut/dracut.conf.d/*.conf` by default.

## Usage
- `dracut` : generate an initramfs image for the current kernel without overriding any options
- `dracut -f` : generate an initramfs image for the current kernel and overwrite the existing one
- `dracut --kver kernel_version` : generate an initramfs image for a specific kernel
- `dracut --list-modules` : list available modules

## Categories
system, containers, media

## Links
- More information: https://github.com/dracut-ng/dracut-ng/blob/main/man/dracut.8.adoc
- tldr page: https://tldr.inbrowser.app/pages/linux/dracut
- Wikipedia: https://en.wikipedia.org/wiki/Dracut_(software)
