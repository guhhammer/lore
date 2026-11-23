# gpu-screen-recorder

Record the screen and encode the video with a GPU.

## Usage
- `gpu-screen-recorder -w portal -o path/to/video.mp4` : select a source using a desktop portal and record it
- `gpu-screen-recorder -w screen|DP-1|HDMI-A1|... -o path/to/video.mp4` : specify a specific video source
- `gpu-screen-recorder --list-capture-options` : list video capture sources
- `gpu-screen-recorder --list-audio-devices|--list-application-audio` : list audio capture sources
- `gpu-screen-recorder -w screen -r 30 -c mp4 -ro path/to/directory -o whatever` : record using the replay buffer
- `pkill -USR1 -f gpu-screen-recorder` : capture a video from the replay buffer
- `systemctl start --user gpu-screen-recorder` : run `gpu-screen-recorder` in the background

## Categories
desktop, hardware, media

## Links
- More information: https://git.dec05eba.com/gpu-screen-recorder/about/
- tldr page: https://tldr.inbrowser.app/pages/linux/gpu-screen-recorder
