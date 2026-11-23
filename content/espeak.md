# espeak

A multi-lingual software speech synthesizer. Note: `espeak` is considered discontinued, as development has shifted to `espeak-ng`.

## Usage
- `espeak "text"` : speak a phrase aloud
- `echo "text" | espeak` : speak text from `stdin`
- `espeak -f path/to/file` : speak the contents of a file
- `espeak -v voice "text"` : speak using a specific voice
- `espeak -s speed -p pitch "text"` : speak at a specific speed (default is 160) and pitch (default is 50)
- `espeak -w path/to/output.wav "text"` : output the audio to a wAV file instead of speaking it directly
- `espeak --voices` : list all available voices

## Categories
text, files

## Links
- More information: https://manned.org/espeak
- tldr page: https://tldr.inbrowser.app/pages/common/espeak
