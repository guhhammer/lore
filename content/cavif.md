# cavif

Convert PNG/JPEG images to AVIF. Written in Rust.

## Usage
- `cavif path/to/image.jpg` : convert a JPEG file to AVIF, saving it to `file.avif`
- `cavif --quality 1..100 path/to/image.png` : adjust the image quality and convert a PNG file to AVIF
- `cavif path/to/image.jpg --output path/to/output.avif` : specify the output location
- `cavif --overwrite path/to/image.jpg` : overwrite the destination file if it already exists

## See also
- convert

## Categories
files, containers, media

## Links
- More information: https://github.com/kornelski/cavif-rs
- tldr page: https://tldr.inbrowser.app/pages/common/cavif
