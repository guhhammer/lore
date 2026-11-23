# pamon

Play back or record raw or encoded audio streams on a PulseAudio sound server.

## Usage
- `paplay [options] [FILE] parecord [options] [FILE] pacat [options] [FILE] parec [options] [FILE]` : the general form, as the manual gives it
- `pamon [options] [FILE] pacat --help pacat --version` : another form of the same command
- `pamon --version` : show version information
- `pamon --monitor-stream=INDEX` : record from the sink input with index INDEX
- `pamon --volume=VOLUME` : specify the initial playback volume to use
- `pamon --rate=SAMPLERATE` : capture or play back audio with the specified sample rate

## Categories
media, files, disks
