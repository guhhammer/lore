# fonttools subset

Generate subsets of fonts or optimize file sizes.

## Usage
- `fonttools subset path/to/font.ttf --unicodes=U+0000-007F` : subset a TTF font file to the Basic Latin Unicode block
- `fonttools subset path/to/font.ttf --unicodes=U+0000-007F --flavor=woff2` : change the file type to WOFF2
- `fonttools subset path/to/font.ttf --unicodes=U+0000-007F --layout-features=onum,kern` : keep only the onum (oldstyle figures) and kern (kerning) OpenType font features
- `fonttools subset path/to/font.ttf --unicodes=U+0000-007F --output-file=path/to/subset.ttf` : set the output file's name

## Categories
files, media

## Links
- More information: https://fonttools.readthedocs.io/en/latest/subset/index.html
- tldr page: https://tldr.inbrowser.app/pages/linux/fonttools-subset
