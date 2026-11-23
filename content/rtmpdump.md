# rtmpdump

Dump media content streamed over the RTMP protocol.

## Usage
- `rtmpdump -r rtmp://example.com/path/to/video -o file.ext` : download a file
- `rtmpdump -r rtmp://example.com/path/to/video -W http://example.com/player -f "LNX 10,0,32,18" -o file.ext` : download a file from a Flash player
- `rtmpdump -r rtmp://example.com/path/to/video -a app_name -y path/to/video -o file.ext` : specify connection parameters if they are not detected correctly
- `rtmpdump -r rtmp://example.com/path/to/video -p http://example.com/webpage -o file.ext` : download a file from a server that requires a referrer

## Categories
files, network, media

## Links
- More information: https://rtmpdump.mplayerhq.hu/rtmpdump.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/rtmpdump
