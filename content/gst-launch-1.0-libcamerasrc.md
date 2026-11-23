# gst-launch-1.0 libcamerasrc

Read frames from a libcamera device.

## Usage
- `gst-launch-1.0 libcamerasrc ! videoconvert ! autovideosink` : view video in a window
- `gst-launch-1.0 libcamerasrc camera-name=camera_name ! videoconvert ! autovideosink` : specify the device to read from

## Categories
hardware

## Links
- More information: https://libcamera.org/getting-started.html#using-gstreamer-plugin
- tldr page: https://tldr.inbrowser.app/pages/common/gst-launch-1.0-libcamerasrc
