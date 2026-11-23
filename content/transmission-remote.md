# transmission-remote

Remote control utility for `transmission-daemon` and `transmission`.

## Usage
- `transmission-remote hostname -a torrent|url -w /path/to/download_directory` : add a torrent file or magnet link to Transmission and download to a specified directory
- `transmission-remote hostname -w /path/to/download_directory` : change the default download directory
- `transmission-remote hostname -l` : list all torrents
- `transmission-remote hostname -t "1,2" -s -t 3 -S` : start torrent 1 and 2, stop torrent 3
- `transmission-remote hostname -t 1 -r -t 2 -rad` : remove torrent 1 and 2, and also delete local data for torrent 2
- `transmission-remote hostname -t all -S` : stop all torrents
- `transmission-remote hostname -t "1-10,15-20" --move /path/to/new_directory` : move torrents 1-10 and 15-20 to a new directory (which will be created if it does not exist)

## Categories
files, network, system

## Links
- More information: https://manned.org/transmission-remote
- tldr page: https://tldr.inbrowser.app/pages/common/transmission-remote
