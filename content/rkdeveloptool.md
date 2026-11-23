# rkdeveloptool

Flash, dump, and manage boot firmware for Rockchip-based computer devices. You will need to turn on the device into Maskrom/Bootrom mode before connecting it through USB. Some subcommands may require to run as root.

## Usage
- `rkdeveloptool ld` : list all connected Rockchip-based flash devices
- `rkdeveloptool db path/to/bootloader.bin` : initialize the device by forcing it to download and install the bootloader from the specified file
- `rkdeveloptool ul path/to/bootloader.bin` : update the bootloader software with a new one
- `rkdeveloptool wl initial_sector path/to/image.img` : write an image to a GPT-formatted flash partition, specifying the initial storage sector (usually `0x0` alias `0`)
- `rkdeveloptool wlx partition_name path/to/image.img` : write to the flash partition by its user-friendly name
- `rkdeveloptool rd` : reset/reboot the device, exit from the Maskrom/Bootrom mode to boot into the selected flash partition

## Categories
hardware, files, system

## Links
- More information: https://github.com/rockchip-linux/rkdeveloptool
- tldr page: https://tldr.inbrowser.app/pages/common/rkdeveloptool
