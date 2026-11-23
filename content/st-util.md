# st-util

Run GDB (GNU Debugger) server to interact with STM32 ARM Cortex microcontroller.

## Usage
- `st-util -p 4500` : run GDB server on port 4500
- `target extended-remote localhost:4500` : [Interactive] Connect to GDB server within `gdb`
- `load firmware.elf` : [Interactive] Write firmware to device

## Categories
development, hardware, network

## Links
- More information: https://github.com/stlink-org/stlink/blob/testing/doc/man/st-util.md
- tldr page: https://tldr.inbrowser.app/pages/common/st-util
