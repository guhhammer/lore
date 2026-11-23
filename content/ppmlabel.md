# ppmlabel

Add text to a PPM image.

## Usage
- `ppmlabel -x pos_x -y pos_y -t text path/to/input_file.ppm >path/to/output_file.ppm` : add text to a PPM image at the specified location
- `ppmlabel -x pos_x1 -y pos_y1 -t text1 -x pos_x2 -y pos_y2 -t text2 path/to/input_file.ppm >path/to/output_file.ppm` : add multiple texts at different locations
- `ppmlabel -x pos_x -y pos_y -c line_color -b background_color -a tilt -s size -t text path/to/input_file.ppm >path/to/output_file.ppm` : specify the line color, the background color, the tilt, and the size of the added text

## Categories
text, files, media

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmlabel.html
- tldr page: https://tldr.inbrowser.app/pages/common/ppmlabel
