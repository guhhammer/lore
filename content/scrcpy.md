# scrcpy

Display and control your Android device on a desktop.

## Usage
- `scrcpy` : display a mirror of a connected device
- `scrcpy -S -w` : turn the device screen off and prevent it from sleeping while mirroring
- `scrcpy -s 0123456789abcdef|192.168.0.1:5555` : display a mirror of a specific device based on its ID or IP address (find it under the `adb devices` command)
- `scrcpy -t` : show touches on physical device
- `scrcpy -r path/to/file.mp4` : record display screen
- `scrcpy --push-target path/to/directory` : specify the target directory for pushing files to device by drag and drop (non-APK)
- `scrcpy --video-source camera` : view the phone's camera (requires Android 12 or newer)
- `scrcpy --video-source camera --camera-size 1920x1080 --v4l2-sink /dev/video0 --no-playback` : create a Video4Linux2 device from the phone's camera (`v4l2loopback` must be installed)

## See also
- adb connect

## Categories
desktop, hardware, files

## Links
- More information: https://github.com/Genymobile/scrcpy
- tldr page: https://tldr.inbrowser.app/pages/common/scrcpy
- Wikipedia: https://en.wikipedia.org/wiki/Scrcpy
