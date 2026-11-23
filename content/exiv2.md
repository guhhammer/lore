# exiv2

Image metadata manipulation tool.

## Usage
- `exiv2 path/to/file` : print a summary of the image Exif metadata
- `exiv2 -P kt path/to/file` : print all metadata (Exif, IPTC, XMP) with interpreted values
- `exiv2 -P kv path/to/file` : print all metadata with raw values
- `exiv2 -d a path/to/file` : delete all metadata from an image
- `exiv2 -d a -k path/to/file` : delete all metadata, preserving the file timestamp
- `exiv2 -r '%Y%m%d_%H%M%S_:basename:' path/to/file` : rename the file, prepending the date and time from metadata (not from the file timestamp)

## Categories
files, containers, media

## Links
- More information: https://exiv2.org/manpage.html
- tldr page: https://tldr.inbrowser.app/pages/common/exiv2
