# wf-recorder

Screencast for Wayland optionally with audio. By default you need to end the process with `<Ctrl c>`.

## Usage
- `wf-recorder -f output.mp4` : record storing to an MP4 file
- `wf-recorder -a -f /path/to/file_with_audio.webm` : record including audio, both with mic and system sounds
- `wf-recorder -g "$(slurp)"` : select and record a portion of the screen using `slurp`, outputting to default `recording.mp4`

## Categories
media, desktop, processes

## Links
- More information: https://github.com/ammen99/wf-recorder
- tldr page: https://tldr.inbrowser.app/pages/linux/wf-recorder
