# mkvmerge

Merge and extract multimedia streams.

## Usage
- `mkvmerge --identify path/to/file.mkv` : display information about a Matroska file
- `mkvextract tracks path/to/file.mkv 1:path/to/output.webm` : extract the audio from track 1 of a specific file
- `mkvextract tracks path/to/file.mkv 3:path/to/subs.srt` : extract the subtitle from track 3 of a specific file
- `mkvmerge --output path/to/output.mkv path/to/file.mkv path/to/subs.srt` : add a subtitle track to a file

## Categories
files, archives, network

## Links
- More information: https://mkvtoolnix.download/doc/mkvmerge.html
- tldr page: https://tldr.inbrowser.app/pages/common/mkvmerge
- Wikipedia: https://en.wikipedia.org/wiki/MKVToolNix
