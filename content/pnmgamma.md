# pnmgamma

Perform gamma correction on PNM images.

## Usage
- `pnmgamma -bt709tolinear|bt709tosrgb path/to/image.pnm >path/to/output.pnm` : convert the image from BT.709 luminance to radiance or sRGB luminance
- `pnmgamma -lineartobt709|srgbtobt709 path/to/image.pnm >path/to/output.pnm` : convert the image from radiance or sRGB luminance to BT.709 luminance
- `pnmgamma -ga value path/to/image.pnm >path/to/output.pnm` : specify the gamma value used for the gamma transfer function
- `pnmgamma -rg value -gg value -bg value path/to/image.pnm >path/to/output.pnm` : specify the gamma value used for the gamma transfer function per color component

## Categories
media, containers, files

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmgamma.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmgamma
