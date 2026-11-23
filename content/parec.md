# parec

Play back or record raw or encoded audio streams on a PulseAudio sound server.

## Usage
- `paplay [options] [FILE]` : the general form, as the manual gives it
- `parecord [options] [FILE] pacat [options] [FILE]` : another form of the same command
- `parec [options] [FILE] pamon [options] [FILE] pacat --help pacat --version` : another form of the same command
- `parec --version` : show version information
- `parec --monitor-stream=INDEX` : record from the sink input with index INDEX
- `parec --volume=VOLUME` : specify the initial playback volume to use
- `parec --rate=SAMPLERATE` : capture or play back audio with the specified sample rate

## Categories
media, files, disks
