# ffprobe

Multimedia stream analyzer.

## Usage
- `ffprobe -v error -show_streams input.mp4` : display all available stream info for a media file
- `ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1 input.mp4` : display media duration
- `ffprobe -v error -select_streams v:0 -show_entries stream=avg_frame_rate -of default=noprint_wrappers=1:nokey=1 input.mp4` : display the frame rate of a video
- `ffprobe -v error -select_streams v:0 -show_entries stream=width|height -of default=noprint_wrappers=1:nokey=1 input.mp4` : display the width or height of a video
- `ffprobe -v error -select_streams v:0 -show_entries stream=bit_rate -of default=noprint_wrappers=1:nokey=1 input.mp4` : display the average bit rate of a video

## Categories
media, desktop

## Links
- More information: https://ffmpeg.org/ffprobe.html
- tldr page: https://tldr.inbrowser.app/pages/common/ffprobe
