# pbmtextps

Render text as a PBM image using PostScript.

## Usage
- `pbmtextps "Hello World!" >path/to/output.pbm` : render a single line of text as a PBM image
- `pbmtextps -font Times-Roman -fontsize 30 "Hello World!" >path/to/output.pbm` : specify the font and font size
- `pbmtextps -l 70 -t 162 "Hello World!" >path/to/output.pbm` : specify the desired left and top margins
- `pbmtextps -du "Hello World!" >path/to/output.ps` : do not output the rendered text as a PBM image, but a PostScript program that would create this image

## See also
- pbmtext

## Categories
media, containers, text

## Links
- More information: https://netpbm.sourceforge.net/doc/pbmtextps.html
- tldr page: https://tldr.inbrowser.app/pages/common/pbmtextps
