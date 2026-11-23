# datashader_cli

Quick visualization of large datasets using CLI based on datashader.

## Usage
- `datashader_cli points path/to/input.parquet --x pickup_x --y pickup_y path/to/output.png --background black|white|#rrggbb` : create a shaded scatter plot of points and save it to a PNG file and set the background color
- `datashader_cli points path/to/input_data.geo.parquet path/to/output_data.png --geo true` : visualize the geospatial data (supports Geoparquet, shapefile, geojson, geopackage, etc.)
- `datashader_cli points path/to/input_data.geo.parquet path/to/output_data.png --geo true --matplotlib true` : use matplotlib to render the image

## Categories
files

## Links
- More information: https://github.com/wybert/datashader-cli
- tldr page: https://tldr.inbrowser.app/pages/common/datashader_cli
