# speak-ng

A multi-lingual software speech synthesizer.

## Usage
- `speak-ng "text"` : speak a phrase aloud
- `echo "text" | speak-ng` : speak text from `stdin`
- `speak-ng -f path/to/file` : speak the contents of a file
- `speak-ng -v voice "text"` : speak using a specific voice
- `speak-ng -s speed -p pitch "text"` : speak at a specific speed (default is 175) and pitch (default is 50)
- `speak-ng -w path/to/output.wav "text"` : output the audio to a wAV file instead of speaking it directly
- `speak-ng --voices` : list all available voices

## See also
- espeak-ng
- espeak

## Categories
text, files

## Links
- More information: https://github.com/espeak-ng/espeak-ng/blob/master/src/speak-ng.1.ronn
- tldr page: https://tldr.inbrowser.app/pages/common/speak-ng
