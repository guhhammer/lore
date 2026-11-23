# gst-launch-1.0

Build and run a GStreamer pipeline.

## Usage
- `gst-launch-1.0 videotestsrc ! autovideosink` : play test video in a window
- `gst-launch-1.0 audiotestsrc -v ! autoaudiosink` : play test audio and enable verbose output
- `gst-launch-1.0 playbin uri=protocol://host/path/to/file` : play a media file in a window
- `gst-launch-1.0 filesrc location=path/to/file ! file_typedemux ! codec_typedec ! codec_typeenc ! file_typemux ! filesink location=path/to/file` : re-encode a media file
- `gst-launch-1.0 filesrc location=path/to/file ! rtspclientsink location=rtsp://host_ip/path/to/file` : stream a file to an RTSP server
- `gst-launch-1.0 -e videotestsrc ! x264enc ! mp4mux ! filesink location=path/to/file.mp4` : force an End Of Stream event if the pipeline is shut down with `<Ctrl c>` for containers that require finalization such as `mp4`
- `gst-launch-1.0 -e videotestsrc ! x264enc ! element_name. audiotestsrc ! opusenc ! element_name. matroskamux name=element_name ! filesink location=path/to/file.mkv` : multiplex together test video and test audio into a file
- `GST_DEBUG=1..5 GST_DEBUG_DUMP_DOT_DIR=path/to/directory gst-launch-1.0 pipeline` : enable debug output and dump a pipeline into a `.dot` file which can then be rendered with tools like `dot`

## See also
- gst-inspect-1.0
- ffmpeg

## Categories
files, development, media

## Links
- More information: https://gstreamer.freedesktop.org/documentation/tools/gst-launch.html
- tldr page: https://tldr.inbrowser.app/pages/common/gst-launch-1.0
