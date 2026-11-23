# ogrmerge.py

Merge several vector datasets into a single one.

## Usage
- `ogrmerge.py -f GPKG -o path/to/output.gpkg path/to/input1.shp path/to/input2.shp ...` : create a GeoPackage with a layer for each input Shapefile
- `ogrmerge.py -f VRT -o path/to/output.vrt path/to/input1.geojson path/to/input2.geojson ...` : create a virtual datasource (VRT) with a layer for each input GeoJSON
- `ogrmerge.py -single -f GeoJSON -o path/to/output.geojson -src_layer_field_name country source_name path/to/input1.shp path/to/input2.shp ...` : concatenate two vector datasets and store source name of dataset in attribute 'source_name'

## Categories
files, version-control

## Links
- More information: https://gdal.org/en/stable/programs/ogrmerge.html
- tldr page: https://tldr.inbrowser.app/pages/common/ogrmerge.py
