# yt-dlp

A youtube-dl fork with additional features and fixes. Download videos from YouTube and other websites.

## Usage
- `yt-dlp "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : download a video or playlist (with the default options from command below)
- `yt-dlp -F "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : list the available downloadable formats for a video
- `yt-dlp -f "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4]" "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : download a video or playlist using the best MP4 video available (default is "bv\*+ba/b")
- `yt-dlp -x "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : extract audio from a video (requires ffmpeg or ffprobe)
- `yt-dlp -x --audio-format mp3 --audio-quality 0 "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : specify audio format and audio quality of extracted audio (between 0 (best) and 10 (worst), default = 5)
- `yt-dlp -I 2,4:6,-1 "https://youtube.com/playlist?list=PLbzoR-pLrL6pTJfLQ3UwtB-3V4fimdqnA"` : download only the second, fourth, fifth, sixth, and last items in a playlist (the first item is 1, not 0)
- `yt-dlp -o "%(uploader)s/%(playlist)s/%(playlist_index)s - %(title)s.%(ext)s" "https://www.youtube.com/user/TheLinuxFoundation/playlists"` : download all playlists of a YouTube channel/user keeping each playlist in a separate directory
- `yt-dlp -u user -p password -P "path/to/directory" -o "%(playlist)s/%(chapter_number)s - %(chapter)s/%(title)s.%(ext)s" "https://www.udemy.com/java-tutorial"` : download a Udemy course keeping each chapter in a separate directory

## See also
- ytfzf

## Categories
network, media, files

## Links
- More information: https://github.com/yt-dlp/yt-dlp#usage-and-options
- tldr page: https://tldr.inbrowser.app/pages/common/yt-dlp
