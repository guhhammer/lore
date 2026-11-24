# webtorrent

The interface for WebTorrent. Supports magnets, URLs, info hashes, and `.torrent` files.

## Usage
- `webtorrent download "torrent_id"` : download a torrent
- `webtorrent download "torrent_id" --vlc` : stream a torrent to VLC media player
- `webtorrent download "torrent_id" --dlna` : stream a torrent to a Digital Living Network Alliance (DLNA) device
- `webtorrent download "torrent_id" --select` : display a list of files for a specific torrent
- `webtorrent download "torrent_id" --select index` : specify a file index from the torrent to download
- `webtorrent seed path/to/file_or_directory` : seed a specific file or directory
- `webtorrent create path/to/file` : create a new torrent file for the specified file path
- `webtorrent info path/to/file_or_magnet` : display information for a magnet URI or `.torrent` file

## Categories
network, files

## Links
- More information: https://github.com/webtorrent/webtorrent-cli
- tldr page: https://tldr.inbrowser.app/pages/common/webtorrent
- Wikipedia: https://en.wikipedia.org/wiki/WebTorrent
