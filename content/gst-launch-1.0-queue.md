# gst-launch-1.0 queue

Multithread the pipeline and buffer data.

## Usage
- `gst-launch-1.0 videotestsrc ! queue ! autovideosink` : separate a pipeline into multiple threads so that one blocking element doesn't block the rest

## Links
- More information: https://gstreamer.freedesktop.org/documentation/coreelements/queue.html
- tldr page: https://tldr.inbrowser.app/pages/common/gst-launch-1.0-queue
