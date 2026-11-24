# ventoy

A tool to create bootable USB drives using ISO files.

## Usage
- `sudo ventoy -i /dev/sdX` : install Ventoy to a specific drive with the defaults
- `sudo ventoy -i -g /dev/sdX` : install Ventoy with GPT partition style instead of MBR
- `sudo ventoy -i -g -S -L LABEL_NAME /dev/sdX` : install Ventoy with GPT partition style and a custom partition label and secure boot disabled
- `sudo ventoy -i -r SIZE_MB /dev/sdX` : install Ventoy and reserve space at the end of the disk
- `sudo ventoy -I /dev/sdX` : force install Ventoy (overwrites existing installation)
- `sudo ventoy -u /dev/sdX` : update Ventoy on a drive
- `sudo ventoy -l /dev/sdX` : display Ventoy information for a drive
- `sudo ventoy -i -n /dev/sdX` : try non-destructive installation if possible (Ventoy will not reformat the disk)

## Categories
disks, users, packages

## Links
- More information: https://www.ventoy.net/en/doc_start.html#doc_linux_cli
- tldr page: https://tldr.inbrowser.app/pages/linux/ventoy
- Wikipedia: https://en.wikipedia.org/wiki/Ventoy
