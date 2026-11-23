# espeak-ng

A multi-lingual software speech synthesizer.

## Usage
- `espeak-ng "text"` : speak a phrase aloud
- `echo "text" | espeak-ng` : speak text from `stdin`
- `espeak-ng -f path/to/file` : speak the contents of a file
- `espeak-ng -v voice "text"` : speak using a specific voice
- `espeak-ng -s speed -p pitch "text"` : speak at a specific speed (default is 175) and pitch (default is 50)
- `espeak-ng -w path/to/output.wav "text"` : output the audio to a wAV file instead of speaking it directly
- `espeak-ng --voices` : list all available voices

## See also
- speak-ng
- espeak

## Categories
text, files

## Links
- More information: https://github.com/espeak-ng/espeak-ng/blob/master/src/espeak-ng.1.ronn
- tldr page: https://tldr.inbrowser.app/pages/common/espeak-ng
