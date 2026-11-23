# bmaptool

Create or copy block maps intelligently (designed to be faster than `cp` or `dd`).

## Usage
- `bmaptool create -o blockmap.bmap source.img` : output a blockmap file from image file
- `bmaptool copy --bmap blockmap.bmap source.img /dev/sdb` : copy an image file into sdb
- `bmaptool copy --bmap blockmap.bmap source.img.gz /dev/sdb` : copy a compressed image file into sdb
- `bmaptool copy --nobmap source.img /dev/sdb` : copy an image file into sdb without using a blockmap

## Categories
files, containers, media

## Links
- More information: https://manned.org/bmaptool
- tldr page: https://tldr.inbrowser.app/pages/common/bmaptool
