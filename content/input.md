# input

Send event codes or touchscreen gestures to an Android device. This command can only be used through `adb shell`.

## Usage
- `input keyevent event_code` : send an event code for a single character to an Android device
- `input text "text"` : send a text to an Android device (`%s` represents spaces)
- `input tap x_position y_position` : send a single tap to an Android device
- `input swipe x_start y_start x_end y_end duration_in_ms` : send a swipe gesture to an Android device
- `input swipe x_position y_position x_position y_position duration_in_ms` : send a long press to an Android device using a swipe gesture

## Notes
An Android command: run it on a device or emulator, usually through `adb shell`.

## Categories
hardware, shell, text, android

## Links
- tldr page: https://tldr.inbrowser.app/pages/android/input
