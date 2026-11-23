# pnmcomp

This command has been superseded by `pamcomp`.

## Usage
- `pamcomp path/to/overlay.pam path/to/underlay.pam >path/to/output.pam` : overlay two images such with the overlay blocking parts of the underlay
- `pamcomp -ali left|center|right|beyondleft|beyondright -x x_offset path/to/overlay.pam path/to/underlay.pam >path/to/output.pam` : set the horizontal alignment of the overlay
- `pamcomp -va top|middle|bottom|above|below -y y_offset path/to/overlay.pam path/to/underlay.pam >path/to/output.pam` : set the vertical alignment of the overlay
- `pamcomp -o 0.7 path/to/overlay.pam path/to/underlay.pam >path/to/output.pam` : set the opacity of the overlay

## Categories
files

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmcomp.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmcomp
