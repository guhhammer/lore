# gdal_translate

Convert raster data between different formats.

## Usage
- `gdal_translate -of JPEG path/to/input.tif path/to/output.jpeg` : convert a raster dataset to JPEG format
- `gdal_translate -a_srs EPSG:4326 path/to/input.tif path/to/output.tif` : assign a projection to a raster dataset
- `gdal_translate -outsize 40% 40% path/to/input.tif path/to/output.tif` : reduce the size of a raster dataset to a specific fraction
- `gdal_translate path/to/input.tif path/to/output.tif -of COG -co COMPRESS=LZW` : convert a GeoTiff to a Cloud Optimized GeoTiff

## Categories
files

## Links
- More information: https://gdal.org/en/stable/programs/gdal_translate.html
- tldr page: https://tldr.inbrowser.app/pages/common/gdal_translate
