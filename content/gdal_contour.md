# gdal_contour

Create contour lines and polygons from a digital elevation model.

## Usage
- `gdal_contour -a ele -i 100.0 path/to/input.tif path/to/output.gpkg` : create a vector dataset with contour lines spread over a 100-meter interval while attributing the elevation property as "ele"
- `gdal_contour -i 100.0 -p path/to/input.tif path/to/output.gpkg` : create a vector dataset with polygons spread over a 100-meter interval

## Categories
files, text

## Links
- More information: https://gdal.org/en/stable/programs/gdal_contour.html
- tldr page: https://tldr.inbrowser.app/pages/common/gdal_contour
