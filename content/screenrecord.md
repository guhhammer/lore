# screenrecord

Record a video of a mobile display. Note: This command can only be used through `adb shell`.

## Usage
- `screenrecord path/to/file.mp4` : record the screen
- `screenrecord --size 1280x720 path/to/file.mp4` : record the screen at a specific resolution
- `screenrecord --bit-rate 6000000 path/to/file.mp4` : record the screen with a specific bitrate
- `screenrecord --time-limit 180 path/to/file.mp4` : record the screen with a maximum duration (in seconds, 180 seconds is the maximum)
- `screenrecord --help` : display help

## Notes
An Android command: run it on a device or emulator, usually through `adb shell`.

## Categories
media, desktop, files, android

## Links
- More information: https://developer.android.com/tools/adb#screenrecord
- tldr page: https://tldr.inbrowser.app/pages/android/screenrecord
