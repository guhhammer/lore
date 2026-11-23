# adb install

Push packages to a connected Android device or emulator.

## Usage
- `adb install path/to/file.apk` : push an Android application to an emulator/device
- `adb -s serial_number install path/to/file.apk` : push an Android application to a specific emulator/device (overrides `$ANDROID_SERIAL`)
- `adb install -r path/to/file.apk` : reinstall an existing app, keeping its data
- `adb install -d path/to/file.apk` : push an Android application allowing version code downgrade (debuggable packages only)
- `adb install -g path/to/file.apk` : grant all permissions listed in the app manifest
- `adb install --fastdeploy path/to/file.apk` : quickly update an installed package by only updating the parts of the APK that changed

## Categories
packages, files, hardware

## Links
- More information: https://developer.android.com/tools/adb
- tldr page: https://tldr.inbrowser.app/pages/common/adb-install
