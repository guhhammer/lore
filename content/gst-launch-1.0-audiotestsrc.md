# gst-launch-1.0 audiotestsrc

Generate basic audio signals.

## Usage
- `gst-launch-1.0 audiotestsrc ! autoaudiosink` : generate and play a sine wave
- `gst-launch-1.0 audiotestsrc wave=sine|square|saw|white-noise|pink-noise|ticks|... ! autoaudiosink` : define what type of wave to generate

## Categories
media

## Links
- More information: https://gstreamer.freedesktop.org/documentation/audiotestsrc/index.html
- tldr page: https://tldr.inbrowser.app/pages/common/gst-launch-1.0-audiotestsrc
