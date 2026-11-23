# crane append

Push an image based on an (optional) base image. Appends layers containing the contents of the provided tarballs.

## Usage
- `crane append -b image_name` : push image based on a base image
- `crane append -f layer_name1 layer_name2 ...` : push image with appended layer from tarball
- `crane append -t tag_name` : push image with appended layer with new tag
- `crane append -o path/to/tarball` : push resulting image to new tarball
- `crane append --oci-empty-base` : use empty base image of type OCI media instead of Docker
- `crane append --set-base-image-annotations` : annotate resulting image as being based on the base image
- `crane append -h` : display help

## Categories
containers, media

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/crane-append
