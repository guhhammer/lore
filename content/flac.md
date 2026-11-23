# flac

Encode, decode, and test FLAC files.

## Usage
- `flac path/to/file.wav` : encode a WAV file to FLAC (this will create a FLAC file in the same location as the WAV file)
- `flac -o path/to/output.flac path/to/file.wav` : encode a WAV file to FLAC, specifying the output file
- `flac -d -o path/to/output.wav path/to/file.flac` : decode a FLAC file to WAV, specifying the output file
- `flac -t path/to/file.flac` : test a FLAC file for the correct encoding

## Categories
files, development

## Links
- More information: https://xiph.org/flac/documentation_tools_flac.html
- tldr page: https://tldr.inbrowser.app/pages/common/flac
