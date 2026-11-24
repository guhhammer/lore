# sox

Sound eXchange: play, record, and convert audio files. Audio formats are identified by the extension.

## Usage
- `sox -m path/to/input_audio1 path/to/input_audio2 path/to/output_audio` : merge two audio files into one
- `sox path/to/input_audio path/to/output_audio trim start duration` : trim an audio file to the specified times
- `sox --norm path/to/input_audio path/to/output_audio` : normalize an audio file (adjust volume to the maximum peak level, without clipping)
- `sox path/to/input_audio path/to/output_audio reverse` : reverse and save an audio file
- `sox path/to/input_audio -n stat` : print statistical data of an audio file
- `sox -v 2.0 path/to/input_audio path/to/output_audio` : increase the volume of an audio file by 2x

## Categories
media, files

## Links
- More information: https://manned.org/sox
- tldr page: https://tldr.inbrowser.app/pages/common/sox
- Wikipedia: https://en.wikipedia.org/wiki/SoX
