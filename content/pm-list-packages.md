# pm list packages

List installed, known, or filtered packages on an Android device.

## Usage
- `pm list packages` : list all installed packages
- `pm list packages -f` : list all packages and their associated APK file paths
- `pm list packages -d` : only list disabled packages
- `pm list packages -e` : only list enabled packages
- `pm list packages -s` : only list system packages
- `pm list packages -3` : only list [3]rd-party (non-system) packages
- `pm list packages -i` : show the installer for each package

## Notes
An Android command: run it on a device or emulator, usually through `adb shell`.

## Categories
packages, hardware, android

## Links
- More information: https://developer.android.com/tools/adb#pm
- tldr page: https://tldr.inbrowser.app/pages/android/pm-list-packages
