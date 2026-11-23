# fastboot

Communicate with connected Android devices when in bootloader mode (the one place ADB doesn't work).

## Usage
- `fastboot oem unlock` : unlock the bootloader
- `fastboot oem lock` : relock the bootloader
- `fastboot reboot bootloader` : reboot the device from fastboot mode into fastboot mode again
- `fastboot flash path/to/file.img` : flash a given image
- `fastboot flash recovery path/to/file.img` : flash a custom recovery image
- `fastboot devices` : list connected devices
- `fastboot getvar all` : display all information of a device

## Categories
hardware, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/fastboot
