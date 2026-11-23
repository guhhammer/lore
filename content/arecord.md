# arecord

Sound recorder for ALSA soundcard driver.

## Usage
- `arecord -vv -f cd path/to/file.wav` : record a snippet in "CD" quality (finish with `<Ctrl c>` when done)
- `arecord -vv -f cd -d 10 path/to/file.wav` : record a snippet in "CD" quality, with a fixed duration of 10 seconds
- `arecord -vv -f cd -t raw | lame -r - path/to/file.mp3` : record a snippet and save it as an MP3 (finish with `<Ctrl c>` when done)
- `arecord -l` : list all sound cards and digital audio devices
- `arecord -i` : allow interactive interface (e.g. use `<Space>` or `<Enter>` to play or pause)
- `arecord -d 5 test-mic.wav && aplay test-mic.wav && rm test-mic.wav` : test your microphone by recording a 5 second sample and playing it back

## Categories
media, files, development

## Links
- More information: https://manned.org/arecord
- tldr page: https://tldr.inbrowser.app/pages/linux/arecord
