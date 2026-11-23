# jpegtran

Perform lossless transformation of JPEG files.

## Usage
- `jpegtran -f horizontal|vertical path/to/image.jpg >path/to/output.jpg` : mirror an image horizontally or vertically
- `jpegtran -ro 90|180|270 path/to/image.jpg >path/to/output.jpg` : rotate an image 90, 180, or 270 degrees clockwise
- `jpegtran -transpose path/to/image.jpg >path/to/output.jpg` : transpose the image across the upper-left to lower right axis
- `jpegtran -transverse path/to/image.jpg >path/to/output.jpg` : transverse the image across the upper right to lower left axis
- `jpegtran -g path/to/image.jpg >path/to/output.jpg` : convert the image to grayscale
- `jpegtran -crop WxH -outfile path/to/output.jpg path/to/image.jpg` : crop the image to a rectangular region of width `W` and height `H` from the upper-left corner, saving the output to a specific file
- `jpegtran -crop WxH+X+Y path/to/image.jpg >path/to/output.jpg` : crop the image to a rectangular region of width `W` and height `H`, starting at point `X` and `Y` from the upper-left corner

## Categories
files, containers, media

## Links
- More information: https://manned.org/jpegtran
- tldr page: https://tldr.inbrowser.app/pages/linux/jpegtran
