# ogrinfo

List information about an OGR-supported data source.

## Usage
- `ogrinfo --formats` : list supported formats
- `ogrinfo path/to/input.gpkg` : list layers of a data source
- `ogrinfo path/to/input.gpkg layer_name` : get detailed information about a specific layer of a data source
- `ogrinfo -so path/to/input.gpkg layer_name` : show summary information about a specific layer of a data source
- `ogrinfo -so -al path/to/input.gpkg` : show summary of all layers of the data source
- `ogrinfo -where 'attribute_name > 42' path/to/input.gpkg layer_name` : show detailed information of features matching a condition
- `ogrinfo path/to/input.geojson -dialect SQLite -sql "UPDATE input SET attribute_name = 'foo'"` : update a layer in the data source with SQL

## Categories
files, databases

## Links
- More information: https://gdal.org/en/stable/programs/ogrinfo.html
- tldr page: https://tldr.inbrowser.app/pages/common/ogrinfo
