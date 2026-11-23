# gst-launch-1.0 videoscale

Resize video frames.

## Usage
- `gst-launch-1.0 videotestsrc ! videoscale ! autovideosink` : scale video to fit the capabilities of both input and output
- `gst-launch-1.0 videotestsrc ! videoscale ! video/x-raw,width=width,height=height ! autovideosink` : scale video to an arbitrary resolution

## Categories
media

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/gst-launch-1.0-videoscale
