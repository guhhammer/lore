# adb forward

Forward socket connections to a connected Android device or emulator.

## Usage
- `adb forward tcp:local_port tcp:remote_port` : forward a TCP port to the only connected emulator or device
- `adb -s device_id forward tcp:local_port tcp:remote_port` : forward a TCP port to a specific emulator or device (by device ID / serial number)
- `adb forward --list` : list all forwardings
- `adb forward --remove tcp:local_port` : remove a forwarding rule
- `adb forward --remove-all` : remove all forwarding rules

## Categories
network, hardware

## Links
- More information: https://developer.android.com/tools/adb
- tldr page: https://tldr.inbrowser.app/pages/common/adb-forward
