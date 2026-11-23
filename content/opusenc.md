# opusenc

Convert WAV or FLAC audio to Opus.

## Usage
- `opusenc path/to/input.wav path/to/output.opus` : convert WAV to Opus using default options
- `opusenc --bitrate 512 path/to/input.wav path/to/output.opus` : convert stereo audio at the highest quality level
- `opusenc --bitrate 1536 path/to/input.flac path/to/output.opus` : convert 5.1 surround sound audio at the highest quality level
- `opusenc path/to/input.wav --downmix-mono --bitrate 6 path/to/out.opus` : convert speech audio at the lowest quality level

## Categories
files, media

## Links
- More information: https://opus-codec.org/docs/opus-tools/opusenc.html
- tldr page: https://tldr.inbrowser.app/pages/common/opusenc
