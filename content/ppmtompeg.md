# ppmtompeg

Encode an MPEG-1 stream.

## Usage
- `ppmtompeg path/to/parameter_file` : produce an MPEG-1 stream using the parameter file to specify inputs and outputs
- `ppmtompeg -g gop_num path/to/parameter_file` : encode the GOP with the specified number only
- `ppmtompeg -fr first_frame last_frame path/to/parameter_file` : specify the first and last frame to encode
- `ppmtompeg -combine_frames path/to/parameter_file` : combine multiple MPEG frames into a single MPEG-1 stream

## Categories
files, media

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmtompeg.html
- tldr page: https://tldr.inbrowser.app/pages/common/ppmtompeg
