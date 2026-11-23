# adb logcat

Dump a log of system messages.

## Usage
- `adb logcat` : display system logs
- `adb logcat -e regex` : display lines that match a `regex`
- `adb logcat tag:mode *:S` : display logs for a tag in a specific mode (Verbose, Debug, Info, Warning, Error, Fatal, Silent), filtering other tags
- `adb logcat ReactNative:V ReactNativeJS:V *:S` : display logs for React Native applications in Verbose mode Silencing other tags
- `adb logcat *:W` : display logs for all tags with priority level Warning and higher
- `adb logcat --pid process_id` : display logs for a specific PID
- `adb logcat --pid $(adb shell pidof -s package)` : display logs for the process of a specific package
- `adb logcat -v color` : color the log (usually use with filters)

## Categories
desktop, processes, monitoring

## Links
- More information: https://developer.android.com/tools/logcat
- tldr page: https://tldr.inbrowser.app/pages/common/adb-logcat
