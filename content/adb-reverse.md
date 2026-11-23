# adb reverse

Reverse socket connections from a connected Android device or emulator.

## Usage
- `adb reverse --list` : list all reverse socket connections from emulators and devices
- `adb reverse tcp:remote_port tcp:local_port` : reverse a TCP port from the only connected emulator or device to localhost
- `adb -s device_id reverse tcp:remote_port tcp:local_port` : reverse a TCP port from a specific emulator or device (by device ID / serial number) to localhost
- `adb reverse --remove tcp:remote_port` : remove a reverse socket connection from an emulator or device
- `adb reverse --remove-all` : remove all reverse socket connections from all emulators and devices

## Categories
network, hardware

## Links
- More information: https://developer.android.com/tools/adb
- tldr page: https://tldr.inbrowser.app/pages/common/adb-reverse
