# ogr2ogr

Convert geospatial vector data between file formats.

## Usage
- `ogr2ogr -f GPKG path/to/output.gpkg path/to/input.shp` : convert a Shapefile into a GeoPackage
- `ogr2ogr -where 'myProperty > 42' -f GeoJSON path/to/output.geojson path/to/input.geojson` : reduce a GeoJSON to features matching a condition
- `ogr2ogr -s_srs EPSG:4326 -t_srs EPSG:3857 -f GPKG path/to/output.gpkg path/to/input.gpkg` : change coordinate reference system of a GeoPackage from `EPSG:4326` to `EPSG:3857`
- `ogr2ogr -f GPKG path/to/output.gpkg path/to/input.csv -oo X_POSSIBLE_NAMES=longitude -oo Y_POSSIBLE_NAMES=latitude -a_srs EPSG:4326` : convert a CSV file into a GeoPackage, specifying the names of the coordinate columns and assigning a coordinate reference system
- `ogr2ogr -f PostgreSQL PG:dbname="database_name" path/to/input.gpkg` : load a GeoPackage into a PostGIS database
- `ogr2ogr -spat min_x min_y max_x max_y -f GPKG path/to/output.gpkg path/to/input.gpkg` : clip layers of a GeoPackage file to the given bounding box

## Categories
files

## Links
- More information: https://gdal.org/en/stable/programs/ogr2ogr.html
- tldr page: https://tldr.inbrowser.app/pages/common/ogr2ogr
