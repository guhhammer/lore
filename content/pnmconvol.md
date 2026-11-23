# pnmconvol

Convolute a PNM image.

## Usage
- `pnmconvol -matrix=-1,3,-1 path/to/image.pnm >path/to/output.pnm` : convolve a PNM image with the specified convolution matrix
- `pnmconvol -matrixfile path/to/matrix1,path/to/matrix2,... path/to/image.pnm >path/to/output.pnm` : convolve a PNM image with the convolution matrix in the specified files, one for each layer in the input image
- `pnmconvol path/to/matrix.pnm path/to/image.pnm >path/to/output.pnm` : convolve a PNM image with the convolution matrix in the specified PNM file
- `pnmconvol -matrix=-1,3,-1 -normalize path/to/image.pnm >path/to/output.pnm` : normalize the weights in the convolution matrix such that they add up to one

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmconvol.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmconvol
