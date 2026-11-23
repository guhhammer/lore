# pbmtext

Render text as a PBM image.

## Usage
- `pbmtext "Hello World!" >path/to/output.pbm` : render a single line of text as a PBM image
- `echo "Hello\nWorld!" | pbmtext >path/to/output.pbm` : render multiple lines of text as a PBM image
- `pbmtext -f path/to/font.pbm "Hello World!" >path/to/output.pbm` : render text using a custom font supplied as a PBM file
- `echo "Hello\nWorld!" | pbmtext -s 3 -ls 10 >path/to/output.pbm` : specify the number of pixels between characters and lines

## See also
- pbmtextps

## Categories
text, media, files

## Links
- More information: https://netpbm.sourceforge.net/doc/pbmtext.html
- tldr page: https://tldr.inbrowser.app/pages/common/pbmtext
