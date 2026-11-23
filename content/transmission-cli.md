# transmission-cli

A lightweight, command-line BitTorrent client. Note: This command is deprecated, use `transmission-remote` instead.

## Usage
- `transmission-cli url|magnet|path/to/file` : download a specific torrent
- `transmission-cli -w path/to/download_directory url|magnet|path/to/file` : download a torrent to a specific directory
- `transmission-cli --new path/to/source_file_or_directory` : create a torrent file from a specific file or directory
- `transmission-cli -d 50 url|magnet|path/to/file` : specify the download speed limit (in KB/s)
- `transmission-cli -u 50 url|magnet|path/to/file` : specify the upload speed limit (in KB/s)
- `transmission-cli -p port_number url|magnet|path/to/file` : use a specific port for connections
- `transmission-cli -er url|magnet|path/to/file` : force encryption for peer connections
- `transmission-cli -b blocklist_url|path/to/blocklist url|magnet|path/to/file` : use a Bluetack-formatted peer blocklist

## Categories
files, network, text

## Links
- More information: https://manned.org/transmission-cli
- tldr page: https://tldr.inbrowser.app/pages/common/transmission-cli
