# crane pull

Pull remote images by reference and store their contents locally.

## Usage
- `crane pull image_name path/to/tarball` : pull remote image
- `crane pull image_name path/to/tarball --annotate-ref` : preserve image reference used to pull as an annotation when used with `--format=oci`
- `crane pull image_name path/to/tarball -c path/to/cache` : cache image layers in a specific directory
- `crane pull image_name path/to/tarball -format format_name` : specify the format in which to save images (default `tarball`)
- `crane pull -h` : display help

## Categories
containers, media, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/crane-pull
