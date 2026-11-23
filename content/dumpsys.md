# dumpsys

Get information about Android system services. This command can only be used through `adb shell`.

## Usage
- `dumpsys` : get diagnostic output for all system services
- `dumpsys service` : get diagnostic output for a specific system service
- `dumpsys -l` : list all services `dumpsys` can give information about
- `dumpsys service -h` : list service-specific arguments for a service
- `dumpsys --skip service` : exclude a specific service from the diagnostic output
- `dumpsys -t 8` : specify a timeout period in seconds (defaults to 10s)

## Notes
An Android command: run it on a device or emulator, usually through `adb shell`.

## Categories
system, shell, android

## Links
- More information: https://developer.android.com/tools/dumpsys
- tldr page: https://tldr.inbrowser.app/pages/android/dumpsys
