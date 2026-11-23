# gdalwarp

Image reprojection and warping utility.

## Usage
- `gdalwarp -t_srs EPSG:4326 path/to/input.tif path/to/output.tif` : reproject a raster dataset
- `gdalwarp -te min_x min_y max_x max_y -te_srs EPSG:4326 path/to/input.tif path/to/output.tif` : crop a raster dataset by using specific coordinates
- `gdalwarp -cutline path/to/area_to_cut.geojson -crop_to_cutline path/to/input.tif path/to/output.tif` : crop a raster dataset using a vector layer

## Categories
files, containers, media

## Links
- More information: https://gdal.org/en/stable/programs/gdalwarp.html
- tldr page: https://tldr.inbrowser.app/pages/common/gdalwarp
