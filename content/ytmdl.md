# ytmdl

Download songs from YouTube and automatically add metadata. Fetch song information (artist, album, cover art) from iTunes, Spotify, and other sources.

## Usage
- `ytmdl song_name` : download a song by name (with interactive selection)
- `ytmdl -q song_name` : download the first result without prompting
- `ytmdl -o path/to/directory song_name` : download a song to a specific directory
- `ytmdl --url https://www.youtube.com/watch?v=oHg5SJYRHA0` : download a song from a YouTube URL
- `ytmdl --format mp3|m4a|opus song_name` : download a song in a specific format (mp3, m4a, or opus)
- `ytmdl --artist artist_name --album album_name song_name` : download a song with artist and album information
- `ytmdl --list path/to/list.txt` : download a list of songs from a text file
- `ytmdl -h` : display help

## Categories
network, files

## Links
- More information: https://github.com/deepjyoti30/ytmdl#usage
- tldr page: https://tldr.inbrowser.app/pages/common/ytmdl
