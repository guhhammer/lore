# transmission-daemon

Daemon controlled with `transmission-remote` or its web interface.

## Usage
- `transmission-daemon` : start a headless `transmission` session
- `transmission-daemon -c path/to/directory` : start and watch a specific directory for new torrents
- `transmission-daemon -d >path/to/file.json` : dump daemon settings in JSON format
- `transmission-daemon -t -u username -v password -p 9091 -a 127.0.0.1` : start with specific settings for the web interface

## See also
- transmission

## Categories
system, network, files

## Links
- More information: https://manned.org/transmission-daemon
- tldr page: https://tldr.inbrowser.app/pages/common/transmission-daemon
