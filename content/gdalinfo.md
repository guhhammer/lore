# gdalinfo

List various information about a GDAL supported raster dataset.

## Usage
- `gdalinfo --formats` : list all supported raster formats
- `gdalinfo path/to/input.tif` : list information about a specific raster dataset
- `gdalinfo -json path/to/input.tif` : list information about a specific raster dataset in JSON format
- `gdalinfo -hist path/to/input.tif` : show histogram values of a specific raster dataset
- `gdalinfo WMS:https://services.meggsimum.de/geoserver/ows` : list information about a Web Map Service (WMS)
- `gdalinfo WMS:https://services.meggsimum.de/geoserver/ows -sd 4` : list information about a specific dataset of a Web Map Service (WMS)

## Categories
network, system, files

## Links
- More information: https://gdal.org/en/stable/programs/gdalinfo.html
- tldr page: https://tldr.inbrowser.app/pages/common/gdalinfo
