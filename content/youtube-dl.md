# youtube-dl

Download videos from YouTube and other websites.

## Usage
- `youtube-dl 'https://www.youtube.com/watch?v=oHg5SJYRHA0'` : download a video or playlist
- `youtube-dl -F 'https://www.youtube.com/watch?v=Mwa0_nE9H7A'` : list all formats that a video or playlist is available in
- `youtube-dl -f "best[height<=480]" 'https://www.youtube.com/watch?v=oHg5SJYRHA0'` : download a video or playlist at a specific quality
- `youtube-dl -x --audio-format mp3 'url'` : download the audio from a video and convert it to an MP3
- `youtube-dl -f bestvideo+bestaudio 'url'` : download the best quality audio and video and merge them
- `youtube-dl -f mp4 -o "%(playlist_index)s-%(title)s by %(uploader)s on %(upload_date)s in %(playlist)s.%(ext)s" 'url'` : download video(s) as MP4 files with custom filenames
- `youtube-dl --sub-lang en --write-sub 'https://www.youtube.com/watch?v=Mwa0_nE9H7A'` : download a particular language's subtitles along with the video
- `youtube-dl -f "bestaudio" -c -w -i -x --audio-format mp3 -o "%(title)s.%(ext)s" 'url_to_playlist'` : download a playlist and extract MP3s from it

## See also
- yt-dlp
- ytfzf
- you-get

## Categories
network, media

## Links
- More information: https://rg3.github.io/youtube-dl/
- tldr page: https://tldr.inbrowser.app/pages/common/youtube-dl
- Wikipedia: https://en.wikipedia.org/wiki/Youtube-dl
