# gst-launch-1.0 pipewiresrc

Read data from a PipeWire node.

## Usage
- `gst-launch-1.0 pipewiresrc ! autoaudiosink` : listen to the default microphone
- `gst-launch-1.0 pipewiresrc target-object=node_name ! autovideosink` : specify node to record and view video in a window
- `gst-launch-1.0 -e pipewiresrc ! videoconvert ! x264enc ! h264parse ! matroskamux ! filesink location=path/to/file.mkv` : record video into a file
- `gst-launch-1.0 pipewiresrc num-buffers=1 ! videoconvert ! pngenc ! filesink location=path/to/file.png` : capture a screenshot from a video node
- `gst-launch-1.0 -e pipewiresrc ! opusenc ! oggmux ! filesink location=path/to/file.ogg` : record audio into a file
- `gst-launch-1.0 pipewiresrc target-object=node_name stream-properties=props,stream.capture.sink=true ! audioconvert ! fakesink` : record the monitor of a device
- `gst-launch-1.0 -e pipewiresrc do-timestamp=true ! videoconvert ! x264enc ! h264parse ! mux. pipewiresrc do-timestamp=true ! opusenc ! mux. matroskamux name=mux ! filesink location=path/to/file.mkv` : multiplex audio and video into a file

## Categories
media, files

## Links
- More information: https://github.com/PipeWire/pipewire/tree/master/src/gst
- tldr page: https://tldr.inbrowser.app/pages/linux/gst-launch-1.0-pipewiresrc
