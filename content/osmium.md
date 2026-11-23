# osmium

Multipurpose tool for handling OpenStreetMap (OSM) files.

## Usage
- `osmium fileinfo path/to/input.osm` : show file information
- `osmium show path/to/input.osm` : display contents
- `osmium cat path/to/input.osm.pbf -o path/to/output.osm` : convert file format from PBF into XML
- `osmium extract -b min_longitude,min_latitude,max_longitude,max_latitude path/to/input.pbf -o path/to/output.pbf` : extract a geographic region by the given bounding box
- `osmium extract -p path/to/polygon.geojson path/to/input.pbf -o path/to/output.pbf` : extract a geographic region by a GeoJSON file
- `osmium tags-filter path/to/input.pbf amenity=restaurant -o path/to/output.pbf` : filter all objects tagged as "restaurant"
- `osmium tags-filter path/to/input.pbf w/highway -o path/to/output.pbf` : filter for "way" objects tagged as "highway"
- `osmium tags-filter path/to/input.pbf wr/building -o path/to/output.pbf` : filter "way" and "relation" objects tagged as "building"

## Categories
files, archives

## Links
- More information: https://osmcode.org/osmium-tool/manual
- tldr page: https://tldr.inbrowser.app/pages/common/osmium
