# streamlink

Extract streams from various services and pipes them into a video player of choice.

## Usage
- `streamlink example.com/stream` : attempt to extract streams from the URL specified, and if it's successful, print out a list of available streams to choose from
- `streamlink example.com/stream 720p60` : open a stream with the specified quality
- `streamlink example.com/stream best|worst` : select the highest or lowest available quality
- `streamlink -p mpv example.com/stream best` : use a specific player to feed stream data to (VLC is used by default if found)
- `streamlink --hls-start-offset HH:MM:SS example.com/stream best` : skip a specific amount of time from the beginning of the stream. For live streams, this is a negative offset from the end of the stream (rewind)
- `streamlink --hls-live-restart example.com/stream best` : skip to the beginning of a live stream, or as far back as possible
- `streamlink -o path/to/file.ts example.com/stream best` : write stream data to a file instead of playing it
- `streamlink -r path/to/file.ts example.com/stream best` : open the stream in the player, while at the same time writing it to a file

## Categories
media, files, archives

## Links
- More information: https://streamlink.github.io/cli.html#command-line-usage
- tldr page: https://tldr.inbrowser.app/pages/common/streamlink
- Wikipedia: https://en.wikipedia.org/wiki/Streamlink
