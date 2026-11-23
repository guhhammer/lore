# pnmmontage

Create a montage from multiple PNM images.

## Usage
- `pnmmontage path/to/image1.pnm path/to/image2.pnm ... >path/to/output.pnm` : produce a packing of the specified images
- `pnmmontage -0..9 path/to/image1.pnm path/to/image2.pnm ... >path/to/output.pnm` : specify the quality of the packing (Note: Larger values produce smaller packings but take longer to compute.)
- `pnmmontage -qua p path/to/image1.pnm path/to/image2.pnm ... >path/to/output.pnm` : produce a packing that is not larger than `p` percent of the optimal packing
- `pnmmontage -d path/to/datafile path/to/image1.pnm path/to/image2.pnm ... >path/to/output.pnm` : write the positions of the input files within the packed image to a machine-readable file

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmmontage.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmmontage
