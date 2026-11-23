# crane tag

Efficiently tag a remote image without downloading it, which differs from the `copy` command. It skips the layer existence checks because we know the manifest already exists making it slightly faster.

## Usage
- `crane tag image_name tag_name` : tag remote image
- `crane tag -h` : display help

## Categories
containers, media, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/crane-tag
