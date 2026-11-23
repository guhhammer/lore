# logcat

Dump a log of system messages, including stack traces when an error occurred, and information messages logged by applications.

## Usage
- `logcat` : display system logs
- `logcat -f path/to/file` : write system logs to a file
- `logcat --regex regex` : display lines that match a `regex`
- `logcat --pid process_id` : display logs for a specific PID
- `logcat --pid $(pidof -s package)` : display logs for the process of a specific package

## Notes
An Android command: run it on a device or emulator, usually through `adb shell`.

## Categories
desktop, processes, text, android

## Links
- More information: https://developer.android.com/tools/logcat
- tldr page: https://tldr.inbrowser.app/pages/android/logcat
