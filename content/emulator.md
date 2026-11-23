# emulator

Manage Android emulators.

## Usage
- `emulator -avd name` : start an Android emulator device
- `emulator -avd name -webcam-list` : display the webcams on your development computer that are available for emulation
- `emulator -avd name -camera-back none|emulated|webcamN` : start an emulator overriding the facing back camera setting (use `-camera-front` for front camera)
- `emulator -avd name -netspeed gsm|hscsd|gprs|edge|hsdpa|lte|evdo|full` : start an emulator, with a maximum network speed
- `emulator -avd name -netdelay gsm|hscsd|gprs|edge|hsdpa|lte|evdo|none` : start an emulator with network latency
- `emulator -avd name -http-proxy http://example.com:80` : start an emulator, making all TCP connections through a specified HTTP/HTTPS proxy (port number is required)
- `emulator -avd name -sdcard path/to/sdcard.img` : start an emulator with a given SD card partition image file
- `emulator -help` : display help

## Categories
network

## Links
- More information: https://developer.android.com/studio/run/emulator-commandline
- tldr page: https://tldr.inbrowser.app/pages/common/emulator
