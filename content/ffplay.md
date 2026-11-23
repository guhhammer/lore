# ffplay

A simple and portable media player using the FFmpeg libraries and the SDL library.

## Usage
- `ffplay path/to/file` : play a media file
- `ffplay -nodisp path/to/file` : play audio from a media file without a GUI
- `ffmpeg -i path/to/file -c copy -f media_format - | ffplay -` : play media passed by `ffmpeg` through `stdin`
- `ffplay -flags2 +export_mvs -vf codecview=mv=pf+bf+bb path/to/file` : play a video and show motion vectors in real time
- `ffplay -vf select="eq(pict_type\,PICT_TYPE_I)" path/to/file` : show only video keyframes

## Categories
files, development, media

## Links
- More information: https://ffmpeg.org/ffplay-all.html
- tldr page: https://tldr.inbrowser.app/pages/common/ffplay
