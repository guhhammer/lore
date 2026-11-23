# rfkill

Enable and disable wireless devices.

## Usage
- `rfkill` : list devices
- `rfkill -o ID,TYPE,DEVICE,...` : filter by columns
- `rfkill block bluetooth|wifi|gps|nfc|...` : block devices by type
- `rfkill unblock bluetooth|wifi|gps|nfc|...` : unblock devices by type
- `rfkill -J` : output in JSON format

## Categories
hardware, network

## Links
- More information: https://manned.org/rfkill
- tldr page: https://tldr.inbrowser.app/pages/linux/rfkill
