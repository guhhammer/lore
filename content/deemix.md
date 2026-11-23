# deemix

A barebone deezer downloader library built from the ashes of Deezloader Remix. It can be used as a standalone CLI app or implemented in a UI using the API.

## Usage
- `deemix https://www.deezer.com/us/track/00000000` : download a track or playlist
- `deemix --bitrate FLAC|MP3 url` : download track/playlist at a specific bitrate
- `deemix --bitrate bitrate --path path/to/directory url` : download to a specific path
- `deemix --portable --bitrate bitrate --path path/to/directory url` : create a portable deemix configuration file in the current directory

## Categories
files, network, development

## Links
- More information: https://gitlab.com/RemixDev/deemix-py
- tldr page: https://tldr.inbrowser.app/pages/common/deemix
