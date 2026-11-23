# gst-launch-1.0 capsfilter

Filter pipeline capabilities.

## Usage
- `gst-launch-1.0 videotestsrc ! capsfilter caps=video/x-raw,format=GRAY8 ! videoconvert ! autovideosink` : filter to only allow greyscale format
- `gst-launch-1.0 videotestsrc ! video/x-raw,format=GRAY8 ! videoconvert ! autovideosink` : do the same but in shortform
- `gst-launch-1.0 videotestsrc ! video/x-raw,width=640,height=480 ! videoconvert ! autovideosink` : limit video capabilities to specific dimensions
- `gst-launch-1.0 videotestsrc ! video/x-raw,framerate=30/1 ! videoconvert ! autovideosink` : specify a specific framerate

## Categories
media

## Links
- More information: https://gstreamer.freedesktop.org/documentation/coreelements/capsfilter.html
- tldr page: https://tldr.inbrowser.app/pages/common/gst-launch-1.0-capsfilter
