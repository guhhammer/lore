# speaker-test

Speaker test tone generator for ALSA.

## Usage
- `speaker-test` : test the default speakers with pink noise
- `speaker-test -t wav` : test the default speakers with a predefined WAV file
- `speaker-test -t wav -c 1..8` : test multichannel speaker configurations
- `speaker-test -t wav -w path/to/file.wav` : test the default speakers with a specific WAV file
- `speaker-test -t sine -f frequency` : test the default speakers with a sine wave

## See also
- aplay
- arecord
- amixer

## Categories
development, files

## Links
- More information: https://manned.org/speaker-test
- tldr page: https://tldr.inbrowser.app/pages/linux/speaker-test
