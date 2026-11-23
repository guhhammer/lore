# timidity

Play and convert MIDI files.

## Usage
- `timidity path/to/file.mid` : play a MIDI file
- `timidity --l path/to/file.mid` : play a MIDI file in a loop
- `timidity --force-keysig=-flats|+sharps path/to/file.mid` : play a MIDI file in a specific key (0 = C major/A minor, -1 = F major/D minor, +1 = G major/E minor, etc.)
- `timidity --output-mode=w --output-file=path/to/file.wav path/to/file.mid` : convert a MIDI file to PCM (WAV) audio
- `timidity --output-mode=F --output-file=path/to/file.flac path/to/file.mid` : convert a MIDI file to FLAC audio

## Categories
files

## Links
- More information: https://manned.org/timidity
- tldr page: https://tldr.inbrowser.app/pages/common/timidity
