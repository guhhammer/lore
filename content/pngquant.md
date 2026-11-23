# pngquant

PNG converter and lossy image compressor.

## Usage
- `pngquant path/to/file.png` : compress a specific PNG as much as possible and write result to a new file
- `pngquant --ext .png -f path/to/file.png` : compress a specific PNG and override original
- `pngquant -Q 0..100 path/to/file.png` : try to compress a specific PNG with custom quality (skip if below the min value)
- `pngquant 64 path/to/file.png` : compress a specific PNG with the number of colors reduced to 64
- `pngquant --skip-if-larger path/to/file.png` : compress a specific PNG and skip if the file is larger than the original
- `pngquant --strip path/to/file.png` : compress a specific PNG and remove metadata
- `pngquant path/to/file.png -o path/to/file.png` : compress a specific PNG and save it to the given path
- `pngquant -v path/to/file.png` : compress a specific PNG and show progress

## Categories
files, archives, containers

## Links
- More information: https://manned.org/pngquant
- tldr page: https://tldr.inbrowser.app/pages/common/pngquant
