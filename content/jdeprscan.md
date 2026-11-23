# jdeprscan

Static analysis tool that scans a jar file (or some other aggregation of class files) for uses of deprecated API elements.

## Usage
- `jdeprscan [options] {dir|jar|class} options See Options for the jdeprscan Command dir|jar|class` : the general form, as the manual gives it
- `jdeprscan --class-path path` : provides a search path for resolution of dependent classes
- `jdeprscan --for-removal` : limits scanning or listing to APIs that are deprecated for removal
- `jdeprscan --full-version` : prints out the full version string of the tool
- `jdeprscan --version` : prints out the abbreviated version string of the tool

## Categories
files
