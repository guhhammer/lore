# hdiutil

Utility to create and manage disk images.

## Usage
- `hdiutil attach path/to/image_file` : mount an image
- `hdiutil detach /Volumes/volume_file` : unmount an image
- `hdiutil info` : list mounted images
- `hdiutil makehybrid -o path/to/output_file path/to/directory` : create an ISO image from the contents of a directory

## Categories
containers, media, files, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/hdiutil.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/hdiutil
