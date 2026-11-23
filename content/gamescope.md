# gamescope

A micro-compositor used as a game layer.

## Usage
- `gamescope -- program` : run a program with gamescope on the terminal
- `gamescope -- %command%` : run a game with gamescope through Steam
- `gamescope -h 720 -H 1440 -S integer -- command` : upscale a 720p game to 1440p with integer scaling
- `gamescope -r 30 -- command` : limit a vsynced game to 30 FPS
- `gamescope -e -- /usr/bin/steam -tenfoot` : launch Steam in Big Picture Mode and integrate with gamescope
- `gamescope -O HDMI-A-1,DP-3,... -- program` : specify which display to prefer
- `<Super f>` : toggle fullscreen
- `gamescope --help` : display help

## See also
- cage

## Links
- More information: https://github.com/ValveSoftware/gamescope#keyboard-shortcuts
- tldr page: https://tldr.inbrowser.app/pages/linux/gamescope
