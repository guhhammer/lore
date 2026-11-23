# gst-launch-1.0 pipewiresink

Create a PipeWire node to output data from.

## Usage
- `gst-launch-1.0 videotestsrc ! pipewiresink mode=provide` : output test video
- `gst-launch-1.0 videotestsrc ! pipewiresink mode=provide client-name=node_name` : give the PipeWire node a name

## Links
- More information: https://github.com/PipeWire/pipewire/tree/master/src/gst
- tldr page: https://tldr.inbrowser.app/pages/linux/gst-launch-1.0-pipewiresink
