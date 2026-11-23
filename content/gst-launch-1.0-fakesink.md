# gst-launch-1.0 fakesink

Dummy sink that swallows everything. Useful for testing what part of a pipeline is breaking.

## Usage
- `gst-launch-1.0 videotestsrc ! fakesink` : consume data from a pipeline without outputting it
- `gst-launch-1.0 -v videotestsrc ! fakesink silent=false` : print information on the received data

## Links
- More information: https://gstreamer.freedesktop.org/documentation/coreelements/fakesink.html
- tldr page: https://tldr.inbrowser.app/pages/common/gst-launch-1.0-fakesink
