# adb

Android Debug Bridge: communicate with an Android emulator instance or connected Android devices. Some subcommands such as `shell` have their own usage documentation.

## Usage
- `adb start-server` : check whether the adb server process is running and start it
- `adb kill-server` : terminate the adb server process
- `adb shell` : start a remote shell in the target emulator/device instance
- `adb install -r path/to/file.apk` : push an Android application to an emulator/device
- `adb pull path/to/device_file_or_directory path/to/local_destination_directory` : copy a file/directory from the target device
- `adb push path/to/local_file_or_directory path/to/device_destination_directory` : copy a file/directory to the target device
- `adb devices` : list all connected devices
- `adb -s device_id shell` : specify which device to send commands to if there are multiple devices

## Categories
files, hardware, shell

## Links
- More information: https://developer.android.com/tools/adb
- tldr page: https://tldr.inbrowser.app/pages/common/adb
