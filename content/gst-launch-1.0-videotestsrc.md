# gst-launch-1.0 videotestsrc

Produce test video data.

## Usage
- `gst-launch-1.0 videotestsrc ! autovideosink` : produce a video with SMPTE color bars
- `gst-launch-1.0 videotestsrc pattern=smpte|snow|green|ball|circular|... ! autovideosink` : define what type of test video to produce

## Categories
media, development

## Links
- More information: https://gstreamer.freedesktop.org/documentation/videotestsrc/index.html
- tldr page: https://tldr.inbrowser.app/pages/common/gst-launch-1.0-videotestsrc
