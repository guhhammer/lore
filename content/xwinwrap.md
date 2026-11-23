# xwinwrap

Run a player or a program as desktop background.

## Usage
- `xwinwrap -b -nf -ov -- mpv --wid wid --loop --no-audio --no-resume-playback --panscan=1.0 path/to/video.mp4` : run a video using mpv
- `xwinwrap -b -nf -fs -ov -- mpv --wid wid --loop --no-audio --no-resume-playback --panscan=1.0 path/to/video.mp4` : run a video in fullscreen using mpv
- `xwinwrap -b -nf -ov -o 0.8 --- mpv --wid wid --loop --no-audio --no-resume-playback --panscan=1.0 path/to/video.mp4` : run a video using mpv with 80% opacity
- `xwinwrap -g 1600x900+1920 -b -nf -ov -- mpv --wid wid --loop --no-audio --no-resume-playback --panscan=1.0 path/to/video.mkv` : run a video using mpv in a second monitor 1600x900 with 1920 offset on X-axis

## Categories
media, files, desktop

## Links
- More information: https://github.com/r00tdaemon/xwinwrap
- tldr page: https://tldr.inbrowser.app/pages/linux/xwinwrap
