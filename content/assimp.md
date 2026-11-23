# assimp

Client for the Open Asset Import Library. Supports loading of 40+ 3D file formats, and exporting to several popular 3D formats.

## Usage
- `assimp listext` : list all supported import formats
- `assimp listexport` : list all supported export formats
- `assimp export input_file.stl output_file.obj` : convert a file to one of the supported output formats, using the default parameters
- `assimp export input_file.stl output_file.obj parameters` : convert a file using custom parameters (the `dox_cmd.h` file in assimp's source code lists available parameters)
- `assimp info path/to/file` : display a summary of a 3D file's contents
- `assimp help` : display help
- `assimp subcommand --help` : display help for a specific subcommand

## Categories
files, development, desktop

## Links
- More information: https://manned.org/assimp
- tldr page: https://tldr.inbrowser.app/pages/common/assimp
