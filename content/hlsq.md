# hlsq

Display HLS manifests with color and basic filtering.

## Usage
- `curl --silent url | hlsq` : view an HLS manifest from a URL
- `cat path/to/file.m3u8 | hlsq` : view an HLS manifest from a file
- `hlsq -watch -url url` : continuously refetch a URL and update the output
- `cat path/to/file.m3u8 | hlsq -query 'type = SUBTITLES'` : filter a multivariant playlist by an attribute's string value
- `cat path/to/file.m3u8 | hlsq -query 'bandwidth > 1000000'` : filter a multivariant playlist by an attribute's numeric value

## Categories
files, network, desktop

## Links
- More information: https://github.com/soldiermoth/hlsq/
- tldr page: https://tldr.inbrowser.app/pages/common/hlsq
