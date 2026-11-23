# mpv

An audio/video player based on MPlayer.

## Usage
- `mpv url|path/to/file` : play a video or audio from a URL or file
- `<ArrowLeft>|<ArrowRight>` : jump backward/forward 5 seconds
- `<ArrowDown>|<ArrowUp>` : jump backward/forward 1 minute
- `<[>|<]>` : decrease or increase playback speed by 10%
- `mpv --sub-file=path/to/file` : add subtitles from a file
- `<s>` : take a screenshot of the current frame (saved to `mpv-shotNNNN.jpg` by default)
- `mpv --speed 0.01..100 path/to/file` : play a file at a specified speed (1 by default)
- `mpv --profile profile_name path/to/file` : play a file using a profile defined in the `mpv.conf` file

## See also
- mplayer
- vlc

## Categories
files, media, network

## Links
- More information: https://mpv.io/manual/stable/
- tldr page: https://tldr.inbrowser.app/pages/common/mpv
