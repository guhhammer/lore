# abroot

Utility providing full immutability and atomicity by transacting between 2 root partition states (A<=>B). Updates are performed using OCI images, to ensure that the system is always in a consistent state.

## Usage
- `sudo abroot pkg add package` : add packages to the local image (Note: After executing this command, you need to apply these changes.)
- `sudo abroot pkg remove package` : remove packages from the local image (Note: After executing this command, you need to apply these changes.)
- `sudo abroot pkg list` : list packages in the local image
- `sudo abroot pkg apply` : apply changes in the local image (Note: You need to reboot your system for these changes to be applied)
- `sudo abroot rollback` : roll back your system to previous state
- `sudo abroot kargs edit|show` : edit/View kernel parameters
- `sudo abroot status` : display status
- `abroot -h` : display help

## Categories
containers, media, users

## Links
- More information: https://docs.vanillaos.org/docs/en/abroot-manpage
- tldr page: https://tldr.inbrowser.app/pages/linux/abroot
