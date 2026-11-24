# aplay

Sound player for ALSA soundcard driver.

## Usage
- `aplay path/to/file` : play a specific file (sampling rate, bit depth, etc. will be automatically determined for the file format)
- `aplay -d 10 -r 2500 path/to/file` : play the first 10 seconds of a specific file at 2500 Hz
- `aplay -c 1 -t raw -r 22050 -f mu_law path/to/file` : play the raw file as a 22050 Hz, mono, 8-bit, Mu-Law `.au` file
- `aplay -l` : list available audio devices

## Categories
files, media

## Links
- More information: https://manned.org/aplay
- tldr page: https://tldr.inbrowser.app/pages/linux/aplay
- Wikipedia: https://en.wikipedia.org/wiki/Aplay
