# ffprobe

Multimedia Stream Analysierer.

## Usage
- `ffprobe -v error -show_streams datei.mp4` : zeige alle verfügbaren Stream-Informationen einer Medien-Datei an
- `ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1 datei.mp4` : zeige die Spieldauer an
- `ffprobe -v error -select_streams v:0 -show_entries stream=avg_frame_rate -of default=noprint_wrappers=1:nokey=1 datei.mp4` : zeige die Bildfrequenz eines Videos an
- `ffprobe -v error -select_streams v:0 -show_entries stream=width|height -of default=noprint_wrappers=1:nokey=1 datei.mp4` : zeige die Breite (width) oder Höhe (height) eines Videos an
- `ffprobe -v error -select_streams v:0 -show_entries stream=bit_rate -of default=noprint_wrappers=1:nokey=1 datei.mp4` : zeige die durchschnittliche Bitrate eines Videos an

## Links
- More information: https://ffmpeg.org/ffprobe.html
