# crane copy

Efficiently copy a remote image from source to target while retaining the digest value.

## Usage
- `crane cp source target` : copy an image from source to target
- `crane cp source target -a` : copy all tags
- `crane cp source target -j int` : set the maximum number of concurrent copies, defaults to GOMAXPROCS
- `crane cp source target -n` : avoid overwriting existing tags in target
- `crane cp -h` : display help

## Categories
files, containers, media

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/crane-copy
