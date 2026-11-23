# gst-launch-1.0 v4l2src

Read frames from a Video4Linux2 device.

## Usage
- `gst-launch-1.0 v4l2src device=/dev/video0 ! autovideosink` : view video in a window
- `gst-launch-1.0 v4l2src device=/dev/video0 ! videoconvert ! pipewiresink mode=provide stream-properties="properties,media.class=Video/Source" client-name="Virtual Camera"` : create a PipeWire node out of a v4l2 device

## Categories
hardware, media

## Links
- More information: https://gstreamer.freedesktop.org/documentation/video4linux2/v4l2src.html
- tldr page: https://tldr.inbrowser.app/pages/linux/gst-launch-1.0-v4l2src
