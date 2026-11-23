# screencap

Take a screenshot of a mobile display. Note: This command can only be used through `adb shell`.

## Usage
- `screencap path/to/image.png` : take a screenshot
- `screencap -p` : print file contents to `stdout` as PNG
- `adb shell screencap -p >path/to/image.png` : take a screenshot and save it over an `adb` connection
- `screencap -h` : display help

## Notes
An Android command: run it on a device or emulator, usually through `adb shell`.

## Categories
media, shell, desktop, android

## Links
- More information: https://developer.android.com/tools/adb#screencap
- tldr page: https://tldr.inbrowser.app/pages/android/screencap
