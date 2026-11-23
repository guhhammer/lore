# mktorrent

Create BitTorrent metainfo files.

## Usage
- `mktorrent -a tracker_announce_url -l 21 -o path/to/example.torrent path/to/file_or_directory` : create a torrent with 2^21 KB as the piece size
- `mktorrent -p -a tracker_announce_url -l 21 -o path/to/example.torrent path/to/file_or_directory` : create a private torrent with a 2^21 KB piece size
- `mktorrent -c "comment" -a tracker_announce_url -l 21 -o path/to/example.torrent path/to/file_or_directory` : create a torrent with a comment
- `mktorrent -a tracker_announce_url,tracker_announce_url_2 -l 21 -o path/to/example.torrent path/to/file_or_directory` : create a torrent with multiple trackers
- `mktorrent -a tracker_announce_url -w web_seed_url -l 21 -o path/to/example.torrent path/to/file_or_directory` : create a torrent with web seed URLs

## Categories
files

## Links
- More information: https://manned.org/mktorrent
- tldr page: https://tldr.inbrowser.app/pages/common/mktorrent
