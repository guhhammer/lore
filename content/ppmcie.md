# ppmcie

Draw a CIE color chart as a PPM image.

## Usage
- `ppmcie >path/to/output.ppm` : draw a CIE color chart using the REC709 color system as a PPM image
- `ppmcie -cie|ebu|hdtv|ntsc|smpte >path/to/output.ppm` : specify the color system to be used
- `ppmcie -red|green|blue xpos ypos >path/to/output.ppm` : specify the location of the individual illuminants
- `ppmcie -f >path/to/output.ppm` : do not dim the area outside the Maxwell triangle

## Categories
media, containers, files

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmcie.html
- tldr page: https://tldr.inbrowser.app/pages/common/ppmcie
