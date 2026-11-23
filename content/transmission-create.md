# transmission-create

Create BitTorrent `.torrent` files.

## Usage
- `transmission-create -o path/to/example.torrent -t tracker_announce_url -s 2048 path/to/file_or_directory` : create a torrent with a specific piece size (in KB)
- `transmission-create -p -o path/to/example.torrent -t tracker_announce_url -s 2048 path/to/file_or_directory` : create a private torrent with a specific piece size (in KB)
- `transmission-create -o path/to/example.torrent -t tracker_url1 -c comment path/to/file_or_directory` : create a torrent with a comment
- `transmission-create -o path/to/example.torrent -t tracker_url1 -t tracker_url2 path/to/file_or_directory` : create a torrent with multiple trackers
- `transmission-create -h` : display help

## See also
- transmission

## Categories
files

## Links
- More information: https://manned.org/transmission-create
- tldr page: https://tldr.inbrowser.app/pages/common/transmission-create
