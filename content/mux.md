# mux

Intercept and multiplex streams of input events.

## Usage
- `mux -c muxer_name1 muxer_name2 ...` : create a new muxer with a specified name
- `mux -s size` : set the muxer's internal queue size (default is 100)
- `mux -i input_muxer_name` : read input from a named muxer (can be repeated in "switch mode")
- `mux -o output_muxer_name` : write output to a named muxer (can be repeated)

## Links
- More information: https://gitlab.com/interception/linux/tools/-/tree/master#mux
- tldr page: https://tldr.inbrowser.app/pages/linux/mux
