# oxipng

Losslessly improve compression of PNG files.

## Usage
- `oxipng path/to/file.png` : compress a PNG file (overwrites the file by default)
- `oxipng --out path/to/output.png path/to/file.png` : compress a PNG file and save the output to a new file
- `oxipng "*.png"` : compress all PNG files in the current directory using multiple threads
- `oxipng -o 0|1|2|3|4|5|6|max path/to/file.png` : compress a file with a set optimization level (default is 2)
- `oxipng -i off|on|keep path/to/file.png` : set the PNG interlacing type (`off` removes interlacing, `on` applies Adam7 interlacing, `keep` preserves existing interlacing; default is `off`)
- `oxipng -a path/to/file.png` : perform additional optimization on images with an alpha channel
- `oxipng -z -o max path/to/file.png` : use the much slower but stronger Zopfli compressor with max optimization
- `oxipng --strip all path/to/file.png` : strip all non-critical metadata chunks

## Categories
files, archives

## Links
- More information: https://manned.org/oxipng
- tldr page: https://tldr.inbrowser.app/pages/common/oxipng
