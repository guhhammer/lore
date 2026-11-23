# m4b-tool

Merge, split, and manipulate audiobook files with chapters.

## Usage
- `m4b-tool merge path/to/input_directory -o=path/to/merged.m4b` : create an audiobook with the audio files in the input directory
- `m4b-tool merge path/to/input_directory -o=path/to/merged.m4b --use-filenames-as-chapters` : make chapters using the input files' names
- `m4b-tool split path/to/audiobook.m4b -o=path/to/output_directory` : split an audiobook into separate files by chapters
- `m4b-tool split path/to/audiobook.m4b --audio-format mp3 -o=path/to/output_directory` : split an audiobook into MP3 files
- `m4b-tool chapters path/to/audiobook.m4b --adjust-by-silence` : adjust chapters using silence detection

## Categories
files, version-control

## Links
- More information: https://github.com/sandreas/m4b-tool
- tldr page: https://tldr.inbrowser.app/pages/common/m4b-tool
