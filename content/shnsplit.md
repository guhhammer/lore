# shnsplit

Split audio files according to a `.cue` file.

## Usage
- `shnsplit -f path/to/file.cue path/to/file.wav` : split a `.wav` + `.cue` file into multiple files
- `shnsplit -a` : show supported formats
- `shnsplit -f path/to/file.cue -o flac path/to/file.flac` : split a `.flac` file into multiple files
- `shnsplit -f path/to/file.cue path/to/file.wav -t "%n - %a - %t"` : split a `.wav` file into files of the form "track-number - album - title"

## Categories
files, media

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/shnsplit
