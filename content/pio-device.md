# pio device

Manage and monitor PlatformIO devices.

## Usage
- `pio device list` : list all available serial ports
- `pio device list --logical` : list all available logical devices
- `pio device monitor` : start an interactive device monitor
- `pio device monitor -p /dev/ttyUSBX` : start an interactive device monitor and listen to a specific port
- `pio device monitor -b 57600` : start an interactive device monitor and set a specific baud rate (defaults to 9600)
- `pio device monitor --eol CRLF|CR|LF` : start an interactive device monitor and set a specific EOL character (defaults to `CRLF`)
- `<Ctrl t>` : go to the menu of the interactive device monitor

## Categories
hardware, monitoring, network

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/device/
- tldr page: https://tldr.inbrowser.app/pages/common/pio-device
