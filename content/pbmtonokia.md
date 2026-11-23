# pbmtonokia

Convert a PBM image to one of Nokia's Smart Messaging Formats.

## Usage
- `pbmtonokia -f NEX_NOL -net network_operator_code path/to/image.pbm >path/to/output.hex` : convert a PBM image into a Nokia Operator Logo as hexcode
- `pbmtonokia -f NEX_NGG path/to/image.pbm >path/to/output.hex` : convert a PBM image into a Nokia Group Graphic as hexcode
- `pbmtonokia -f NEX_NPM -txt text_message path/to/image.pbm >path/to/output.hex` : convert a PBM image into a Nokia Picture Message with the specified text as hexcode
- `pbmtonokia -f NOL path/to/image.pbm >path/to/output.nol` : convert a PBM image into a Nokia Operator Logo as a NOL file
- `pbmtonokia -f NGG path/to/image.pbm >path/to/output.ngg` : convert a PBM image into a Nokia Group Graphic as an NGG file
- `pbmtonokia -f NPM path/to/image.pbm >path/to/output.npm` : convert a PBM image into a Nokia Picture Message as an NPM file

## Categories
media, containers, files

## Links
- More information: https://netpbm.sourceforge.net/doc/pbmtonokia.html
- tldr page: https://tldr.inbrowser.app/pages/common/pbmtonokia
