# rpi-imager

Flash images onto storage devices.

## Usage
- `rpi-imager --cli path/to/image.zip /dev/sdX` : write a specific image to a specific block device
- `rpi-imager --cli --disable-verify path/to/image.zip /dev/sdX` : write a specific image to a block device, disabling the checksum verification
- `rpi-imager --cli --sha256 expected_hash path/to/image.zip /dev/sdX` : write a specific image to a block device, which will expect a specific checksum when running the verification

## Categories
containers, media, hardware

## Links
- More information: https://github.com/raspberrypi/rpi-imager
- tldr page: https://tldr.inbrowser.app/pages/common/rpi-imager
