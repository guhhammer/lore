# gst-launch-1.0 pulsesrc

Read audio data from a Pulseaudio source.

## Usage
- `gst-launch-1.0 pulsesrc ! autoaudiosink` : listen to the default microphone
- `gst-launch-1.0 pulsesrc device="device_name" ! autoaudiosink` : specify the source by name (can match a substring)
- `gst-launch-1.0 pulsesrc device=@DEFAULT_MONITOR@ ! opusenc ! oggmux ! filesink location=path/to/file.ogg` : record the monitor of the default sink

## Categories
media

## Links
- More information: https://gstreamer.freedesktop.org/documentation/pulseaudio/pulsesrc.html
- tldr page: https://tldr.inbrowser.app/pages/common/gst-launch-1.0-pulsesrc
