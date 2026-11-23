# youtube-dl

Download video's van YouTube en andere websites.

## Usage
- `youtube-dl 'https://www.youtube.com/watch?v=oHg5SJYRHA0'` : download een video of afspeellijst
- `youtube-dl -F 'https://www.youtube.com/watch?v=Mwa0_nE9H7A'` : toon alle formaten waarin een video of afspeellijst beschikbaar is
- `youtube-dl -f "best[height<=480]" 'https://www.youtube.com/watch?v=oHg5SJYRHA0'` : download een video of afspeellijst met een specifieke kwaliteit
- `youtube-dl -x --audio-format mp3 'url'` : download de audio van een video en converteer het naar MP3
- `youtube-dl -f bestvideo+bestaudio 'url'` : download de best mogelijke audio en video en voeg ze samen
- `youtube-dl -f mp4 -o "%(playlist_index)s-%(title)s by %(uploader)s on %(upload_date)s in %(playlist)s.%(ext)s" 'url'` : download video('s) als MP4-bestanden met aangepaste bestandsnamen
- `youtube-dl --sub-lang en --write-sub 'https://www.youtube.com/watch?v=Mwa0_nE9H7A'` : download de ondertiteling van een specifieke taal samen met de video
- `youtube-dl -f "bestaudio" -c -w -i -x --audio-format mp3 -o "%(title)s.%(ext)s" 'url_to_playlist'` : download een afspeellijst en extraheer er MP3's uit

## Links
- More information: https://rg3.github.io/youtube-dl/
