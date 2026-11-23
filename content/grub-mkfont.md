# grub-mkfont

Convert common font file formats into the PF2 format used by GRUB.

## Usage
- `grub-mkfont -o path/to/output.pf2 path/to/font.ttf` : convert a font to PF2
- `grub-mkfont -o path/to/output.pf2 -s 24 path/to/font.ttf` : convert a font at a specific size
- `grub-mkfont -o path/to/output.pf2 -r 0x20-0x7E,0xA0-0xFF path/to/font.ttf` : convert only characters in specific Unicode ranges
- `grub-mkfont -o path/to/output.pf2 -n "font_family_name" path/to/font.ttf` : set the output font family name
- `grub-mkfont -o path/to/output.pf2 -b -a path/to/font.ttf` : convert a font to bold while forcing automatic hinting
- `grub-mkfont -o path/to/output.pf2 --no-hinting --no-bitmap path/to/font.ttf` : disable hinting and ignore embedded bitmap strikes

## Categories
files, media

## Links
- More information: https://manned.org/grub-mkfont
- tldr page: https://tldr.inbrowser.app/pages/linux/grub-mkfont
