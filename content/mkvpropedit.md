# mkvpropedit

Modify properties of existing Matroska files without a complete remux.

## Usage
- `mkvpropedit -d title path/to/file.mkv` : delete title
- `mkvpropedit path/to/file.mkv -e track:s3 -s flag-hearing-impaired=1` : mark subtitle track 3 as SDH - Subtitles for deaf and hearing impaired
- `mkvpropedit path/to/file.mkv -e track:a2 -s flag-default=1` : mark audio track 2 as Default
- `mkvpropedit path/to/file.mkv -e track:v1 -d name` : delete the name of video track 1

## Categories
files, network

## Links
- More information: https://mkvtoolnix.download/doc/mkvpropedit.html
- tldr page: https://tldr.inbrowser.app/pages/linux/mkvpropedit
