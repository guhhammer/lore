# gst-launch-1.0 videorate

Normalize input framerate.

## Usage
- `gst-launch-1.0 videotestsrc ! videorate ! autovideosink` : normalize framerate and apply new timestamps
- `gst-launch-1.0 videotestsrc ! videorate ! video/x-raw,framerate=60/1 ! autovideosink` : normalize framerate to 60 frames per second

## Links
- More information: https://gstreamer.freedesktop.org/documentation/videorate/index.html
- tldr page: https://tldr.inbrowser.app/pages/common/gst-launch-1.0-videorate
