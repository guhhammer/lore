# bless

Set volume boot capability and startup disk options.

## Usage
- `bless --folder /Volumes/Mac OS X/System/Library/CoreServices --bootinfo --bootefi` : bless a volume with only Mac OS X or Darwin, and create the BootX and `boot.efi` files as needed
- `bless --mount /Volumes/Mac OS --setBoot` : set a volume containing either Mac OS 9 and Mac OS X to be the active volume
- `bless --netboot --server bsdp://255.255.255.255` : set the system to NetBoot and broadcast for an available server
- `bless --info --plist` : gather information about the currently selected volume (as determined by the firmware), suitable for piping to a program capable of parsing Property Lists

## Categories
disks, system, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/bless.8.html
- tldr page: https://tldr.inbrowser.app/pages/osx/bless
