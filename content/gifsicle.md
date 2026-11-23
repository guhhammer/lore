# gifsicle

Create, edit, manipulate, and get information about GIF files.

## Usage
- `gifsicle path/to/input.gif -O3 -o path/to/output.gif` : optimize a GIF as a new file
- `gifsicle -b path/to/input.gif -U` : use batch mode (modify each given file in place) and unoptimize a GIF
- `gifsicle path/to/input.gif '#0' >path/to/first_frame.gif` : extract a frame from a GIF
- `gifsicle *.gif -d 10 -l >path/to/output.gif` : make a GIF animation from selected GIFs
- `gifsicle -b path/to/input.gif -O3 --lossy=100 -k 16 -f` : reduce file size using lossy compression
- `gifsicle -b path/to/input.gif --delete '#0-9' '#20-'` : delete the first 10 frames and all frames after frame 20 from a GIF
- `gifsicle -b --crop starting_x,starting_y+rect_widthxrect_height --scale 0.25 --flip-horizontal --rotate-90|180|270 path/to/input.gif` : modify all frames by cropping them to a rectangle, changing their scale, flipping them, and rotating them

## Categories
files

## Links
- More information: https://www.lcdf.org/gifsicle/
- tldr page: https://tldr.inbrowser.app/pages/common/gifsicle
