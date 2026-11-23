# peerflix

Stream video- or audio-based torrents to a media player.

## Usage
- `peerflix "torrent_url|magnet_link"` : stream the largest media file in a torrent
- `peerflix "magnet:?xt=urn:btih:0123456789abcdef0123456789abcdef01234567" -l` : list all streamable files contained in a torrent (given as a magnet link)
- `peerflix "http://example.net/music.torrent" -v` : stream the largest file in a torrent, given as a torrent URL, to VLC
- `peerflix "torrent_url|magnet_link" -m -t subtitle-file.srt` : stream the largest file in a torrent to MPlayer, with subtitles
- `peerflix "torrent_url|magnet_link" -a -s` : stream all files from a torrent to Airplay

## Categories
media, files, network

## Links
- More information: https://github.com/mafintosh/peerflix
- tldr page: https://tldr.inbrowser.app/pages/common/peerflix
