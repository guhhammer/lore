# mp3info

Viewer/editor for ID3v1 (but not ID3v2) tags of MP3 files.

## Usage
- `mp3info path/to/file.mp3` : show all ID3v1 tags of a specific MP3 file
- `mp3info -i path/to/file.mp3` : edit ID3v1 tags interactively
- `mp3info -a "artist_name" -t "song_title" -l "album_title" -y year -c "comment_text" path/to/file.mp3` : set values for ID3v1 tags in a specific MP3 file (artist, title, album, year, and comment)
- `mp3info -n track_number path/to/file.mp3` : set the number of the track in the album for a specific MP3 file
- `mp3info -G` : get a list of valid genres and their numeric codes
- `mp3info -g genre_number path/to/file.mp3` : set the music genre for a specific MP3 file

## Categories
files, editors

## Links
- More information: https://www.ibiblio.org/mp3info/mp3info.html
- tldr page: https://tldr.inbrowser.app/pages/common/mp3info
