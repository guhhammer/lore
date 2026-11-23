# grub-mkrescue

Make a GRUB CD/USB/floppy bootable image.

## Usage
- `grub-mkrescue --output grub.iso .` : create a bootable ISO from the current directory and save it as `grub.iso`
- `grub-mkrescue --directory /usr/lib/grub/i386-pc --output grub.iso path/to/source` : create an ISO using GRUB files from a custom directory
- `grub-mkrescue --compress no|xz|gz|lzo --output grub.iso path/to/source` : use compression for GRUB files when building the image, setting `no` disables compression
- `grub-mkrescue --disable-cli --output grub.iso path/to/source` : disable the GRUB command-line interface in the generated image
- `grub-mkrescue --modules "part_gpt iso9660" --output grub.iso path/to/source` : preload specific GRUB modules into the image
- `grub-mkrescue --output grub.iso -- -volid volume_name path/to/source` : pass additional options directly to `xorriso`
- `grub-mkrescue -?` : display help
- `grub-mkrescue --version` : display version

## Categories
files, containers, media

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/grub-mkrescue
