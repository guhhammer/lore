# gdalbuildvrt

Build Virtual Datasets from a list of existing datasets.

## Usage
- `gdalbuildvrt path/to/output.vrt path/to/input_directory/*.tif` : make a virtual mosaic from all TIFF files contained in a directory
- `gdalbuildvrt -input_file_list path/to/list.txt path/to/output.vrt` : make a virtual mosaic from files whose name is specified in a text file
- `gdalbuildvrt -separate path/to/rgb.vrt path/to/red.tif path/to/green.tif path/to/blue.tif` : make an RGB virtual mosaic from 3 single-band input files
- `gdalbuildvrt -hidenodata -vrtnodata "0 0 255" path/to/output.vrt path/to/input_directory/*.tif` : make a virtual mosaic with blue background color (RGB: 0 0 255)

## Categories
files, development

## Links
- More information: https://gdal.org/en/stable/programs/gdalbuildvrt.html
- tldr page: https://tldr.inbrowser.app/pages/common/gdalbuildvrt
