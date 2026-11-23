# pm

Android Package Manager tool.

## Usage
- `pm list packages` : list installed packages
- `pm install /path/to/app.apk` : install an app package from a given path
- `pm uninstall package` : uninstall a package from the device
- `pm clear package` : clear all app data for a package
- `pm enable package_or_class` : enable a package or component
- `pm disable-user package_or_class` : disable a package or component
- `pm grant package android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : grant a permission for an app
- `pm revoke package android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : revoke a permission for an app

## Notes
An Android command: run it on a device or emulator, usually through `adb shell`.

## Categories
packages, files, android

## Links
- More information: https://developer.android.com/tools/adb#pm
- tldr page: https://tldr.inbrowser.app/pages/android/pm
