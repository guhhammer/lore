# crane push

Push local image contents to a remote registry.

## Usage
- `crane push path/to/tarball image_name` : push local image to remote registry
- `crane push path/to/tarball image_name --image-refs path/to/file` : write a list of published image references to a file
- `crane push path/to/tarball image_name --index` : push a collection of images as a single index (required if path has multiple images)
- `crane push -h` : display help

## Categories
containers, media, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/crane-push
