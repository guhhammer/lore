# ppmtopjxl

Convert a PPM image into an HP PaintJet XL PCL file.

## Usage
- `ppmtopjxl path/to/image.ppm >path/to/output.pjxl` : convert a PPM image into a PJXL file
- `ppmtopjxl -xsi 10cm -ysi 5cm path/to/image.ppm >path/to/output.pjxl` : resize the input image
- `ppmtopjxl -xsh 10pt -ysh 5pt path/to/image.ppm >path/to/output.pjxl` : shift the input image
- `ppmtopjxl -n path/to/image.ppm >path/to/output.pjxl` : do not use the normal TIFF 4.0 compression method

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmtopjxl.html
- tldr page: https://tldr.inbrowser.app/pages/common/ppmtopjxl
