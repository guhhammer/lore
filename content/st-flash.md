# st-flash

Flash binary files to STM32 ARM Cortex microcontrollers.

## Usage
- `st-flash read firmware.bin 0x8000000 4096` : read 4096 bytes from the device starting from 0x8000000
- `st-flash write firmware.bin 0x8000000` : write firmware to device starting from 0x8000000
- `st-flash erase` : erase firmware from device

## Categories
hardware

## Links
- More information: https://github.com/stlink-org/stlink/blob/testing/doc/man/st-flash.md
- tldr page: https://tldr.inbrowser.app/pages/common/st-flash
