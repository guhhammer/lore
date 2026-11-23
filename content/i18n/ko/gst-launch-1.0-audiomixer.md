# gst-launch-1.0 audiomixer

여러 오디오 스트림을 하나로 믹싱.

## Usage
- `gst-launch-1.0 audiotestsrc ! element_name. audiotestsrc ! element_name. audiomixer name=element_name ! fakesink` : 두 개의 오디오 스트림을 함께 믹싱

## Links
- More information: https://gstreamer.freedesktop.org/documentation/audiomixer/audiomixer.html
