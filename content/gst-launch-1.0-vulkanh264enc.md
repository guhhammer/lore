# gst-launch-1.0 vulkanh264enc

Encode video into h264 using the Vulkan API.

## Usage
- `gst-launch-1.0 videotestsrc ! vulkanupload ! vulkanh264enc ! h264parse ! filesink location=path/to/file.h264` : encode h264 video

## Categories
media

## Links
- More information: https://gstreamer.freedesktop.org/documentation/vulkan/vulkanh264enc.html
- tldr page: https://tldr.inbrowser.app/pages/common/gst-launch-1.0-vulkanh264enc
