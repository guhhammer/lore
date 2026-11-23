# chafa

Image printing in the terminal.

## Usage
- `chafa path/to/file` : render an image directly in the terminal
- `chafa -c full path/to/file` : render an image with 24-bit color
- `chafa -c 16 --dither ordered path/to/file` : improve image rendering with small color palettes using dithering
- `chafa --symbols vhalf path/to/file` : render an image, making it appear pixelated
- `chafa -c none --symbols braille path/to/file` : render a monochrome image with only braille characters

## See also
- catimg
- pixterm

## Categories
files, media, containers

## Links
- More information: https://hpjansson.org/chafa/man/
- tldr page: https://tldr.inbrowser.app/pages/common/chafa
