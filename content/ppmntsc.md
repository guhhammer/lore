# ppmntsc

Make the RGB colors in a PPM image compatible with NTSC or PAL color systems.

## Usage
- `ppmntsc path/to/input_file.ppm >path/to/output_file.ppm` : make the RGB colors in a PPM image compatible with NTSC color systems
- `ppmntsc --pal path/to/input_file.ppm >path/to/output_file.ppm` : make the RGB colors in a PPM image compatible with PAL color systems
- `ppmntsc --verb path/to/input_file.ppm >path/to/output_file.ppm` : print the number of illegal pixels in the input image to `stderr`
- `ppmntsc --legalonly|illegalonly|correctedonly path/to/input_file.ppm >path/to/output_file.ppm` : output only legal/illegal/corrected pixels, set other pixels to black

## Categories
media, files, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmntsc.html
- tldr page: https://tldr.inbrowser.app/pages/common/ppmntsc
