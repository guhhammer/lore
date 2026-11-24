# transmission

A simple torrent client. Note: Transmission isn't a command, but a set of commands.

## Usage
- `transmission-daemon` : start a headless `transmission` session
- `transmission-daemon -c path/to/directory` : start and watch a specific directory for new torrents
- `transmission-remote hostname -a torrent|url -w /path/to/download_directory` : add a torrent file or magnet link to Transmission and download to a specified directory
- `transmission-remote hostname -w /path/to/download_directory` : change the default download directory
- `transmission-create -o path/to/example.torrent -t tracker_announce_url -s 2048 path/to/file_or_directory` : create a torrent with a specific piece size (in KB)
- `transmission-create -p -o path/to/example.torrent -t tracker_announce_url -s 2048 path/to/file_or_directory` : create a private torrent with a specific piece size (in KB)
- `transmission-edit -a http://example.com path/to/file.torrent` : add a URL to a torrent's announce list
- `transmission-edit -d http://example.com path/to/file.torrent` : remove a URL from a torrent's announce list

## Categories
files, network

## Links
- More information: https://transmissionbt.com/
- tldr page: https://tldr.inbrowser.app/pages/common/transmission
- Wikipedia: https://en.wikipedia.org/wiki/Transmission_(BitTorrent_client)
