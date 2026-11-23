# mid3v2

Edit audio tags.

## Usage
- `mid3v2 --list-frames path/to/file1.mp3 path/to/file2.mp3 ...` : list all supported ID3v2.3 or ID3v2.4 frames and their meanings
- `mid3v2 --list-genres path/to/file1.mp3 path/to/file2.mp3 ...` : list all supported ID3v1 numeric genres
- `mid3v2 --list path/to/file1.mp3 path/to/file2.mp3 ...` : list all tags in specific files
- `mid3v2 --artist|--album|--song=string path/to/file1.mp3 path/to/file2.mp3 ...` : set specific artist, album, or song information
- `mid3v2 --picture=filename:description:image_type:mime_type path/to/file1.mp3 path/to/file2.mp3 ...` : set specific picture information
- `mid3v2 --year=YYYY path/to/file1.mp3 path/to/file2.mp3 ...` : set specific year information
- `mid3v2 --date=YYYY-MM-DD path/to/file1.mp3 path/to/file2.mp3 ...` : set specific date information

## See also
- id3v2

## Categories
files, media

## Links
- More information: https://mutagen.readthedocs.io/en/latest/man/mid3v2.html
- tldr page: https://tldr.inbrowser.app/pages/common/mid3v2
