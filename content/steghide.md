# steghide

Steganography tool for JPEG, BMP, WAV, and AU file formats.

## Usage
- `steghide embed -cf path/to/image.png -ef path/to/data.txt` : embed data in a PNG, prompting for a passphrase
- `steghide extract -sf path/to/sound.wav` : extract data from a WAV audio file
- `steghide info path/to/file.jpg` : display file information, trying to detect an embedded file
- `steghide embed -cf path/to/image.jpg -ef path/to/data.txt -z 9` : embed data in a JPEG image, using maximum compression
- `steghide encinfo` : get the list of supported encryption algorithms and modes
- `steghide embed -cf path/to/image.jpg -ef path/to/data.txt -e blowfish|... cbc|...` : embed encrypted data in a JPEG image, e.g. with Blowfish in CBC mode

## Categories
files, media, containers

## Links
- More information: https://manned.org/steghide
- tldr page: https://tldr.inbrowser.app/pages/linux/steghide
