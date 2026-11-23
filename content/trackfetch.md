# trackfetch

Download songs listed in a text file or passed as arguments, saving them as tagged MP3, M4A, or Opus files using configurable metadata sources and YouTube audio. Requires `yt-dlp`, `ffmpeg`, and `deno`. Spotify is optional and only used when selected.

## Usage
- `trackfetch path/to/songs.txt` : download every song from a file with one `Artist - Title` per line into the current directory
- `trackfetch path/to/songs.txt -o path/to/directory` : download songs into a specific directory
- `trackfetch "Artist1 - Title1" "Artist2 - Title2" ...` : download one or more tracks passed as arguments instead of a text file
- `trackfetch path/to/songs.txt -s deezer,itunes,musicbrainz` : choose which metadata sources to use (the default is `itunes`)
- `trackfetch path/to/songs.txt -f mp3|m4a|opus` : choose the output format (the default is `m4a`; `m4a` and `opus` keep YouTube's original audio)
- `trackfetch path/to/songs.txt --config path/to/config.ini` : use a config file for the defaults
- `trackfetch path/to/songs.txt --delay 3` : wait a number of seconds between songs to avoid rate limiting
- `cut -d '|' -f 1 path/to/directory/failed.txt >path/to/retry.txt && trackfetch path/to/retry.txt -o path/to/directory` : retry the songs that failed in a previous run

## Categories
files, network, text

## Links
- More information: https://github.com/trackfetch/trackfetch#usage
- tldr page: https://tldr.inbrowser.app/pages/common/trackfetch
