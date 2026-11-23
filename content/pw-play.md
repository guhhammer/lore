# pw-play

This command is an alias of `pw-cat --playback`.

## Usage
- `pw-cat -p path/to/file.wav` : play a WAV file over the default target
- `pw-cat -pm path/to/file.mid` : play a MIDI file
- `pw-cat -pd path/to/file.dsf` : play a DSD file
- `pw-cat -po path/to/file.ac3` : play a compressed audio file using passthrough (requires FFmpeg integration)
- `pw-cat -p -q 0..15 path/to/file.wav` : play a WAV file with a specified resampler quality (4 by default)
- `pw-cat -rm path/to/file.mid` : record a MIDI file
- `pw-cat -r --volume 1.25 path/to/file.wav` : record a sample recording at a volume level of 125%
- `pw-cat -r --rate 6000 path/to/file.wav` : record a sample recording using a different sample rate

## Categories
files, media, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/pw-play
