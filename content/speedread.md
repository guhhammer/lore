# speedread

A Spritz-alike speed reader. Show text using per-word Rapid Serial Visual Presentation (RSVP).

## Usage
- `cat path/to/file.txt | speedread -wpm 250` : read a text file at a specific speed
- `cat path/to/file.txt | speedread -resume 5` : resume from a specific line
- `cat path/to/file.txt | speedread -multiword` : show multiple words at a time
- `<[>` : slow down by 10% during the reading session
- `<]>` : speed up by 10% during the reading session
- `<Space>` : pause, and show the last few lines as context

## Categories
text, files

## Links
- More information: https://github.com/pasky/speedread
- tldr page: https://tldr.inbrowser.app/pages/linux/speedread
