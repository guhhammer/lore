# pamfix

Fix errors in PAM, PBM, PGM, and PPM files.

## Usage
- `pamfix -t path/to/corrupted.ext >path/to/output.ext` : fix a Netpbm file that is missing its last part
- `pamfix -cl path/to/corrupted.ext >path/to/output.ext` : fix a Netpbm file where pixel values exceed the image's `maxval` by lowering the offending pixels' values
- `pamfix -ch path/to/corrupted.pam|pbm|pgm|ppm >path/to/output.pam|pbm|pgm|ppm` : fix a Netpbm file where pixel values exceed the image's `maxval` by increasing it

## See also
- pamfile
- pamvalidate

## Categories
files

## Links
- More information: https://netpbm.sourceforge.net/doc/pamfix.html
- tldr page: https://tldr.inbrowser.app/pages/common/pamfix
