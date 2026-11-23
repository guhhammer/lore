# youtube-dl

Descarrega videos de YouTube i altres pàgines web.

## Usage
- `youtube-dl 'https://www.youtube.com/watch?v=oHg5SJYRHA0'` : descarrega un vídeo o playlist
- `youtube-dl -F 'https://www.youtube.com/watch?v=Mwa0_nE9H7A'` : llista tots els formats en el que es troba disponible un vídeo o playlist
- `youtube-dl -f "best[height<=480]" 'https://www.youtube.com/watch?v=oHg5SJYRHA0'` : descarrega un vídeo o playlist en una qualitat específica
- `youtube-dl -x --audio-format mp3 'url'` : descarrega l'àudio d'un vídeo i converteix-lo a MP3
- `youtube-dl -f bestvideo+bestaudio 'url'` : descarrega l'àudio i el vídeo de major qualitat i fusiona'ls
- `youtube-dl -f mp4 -o "%(playlist_index)s-%(title)s by %(uploader)s on %(upload_date)s in %(playlist)s.%(ext)s" 'url'` : descarrega vídeo(s) com a fitxers MP4 amb un nom específic
- `youtube-dl --sub-lang en --write-sub 'https://www.youtube.com/watch?v=Mwa0_nE9H7A'` : descarrega els subtítols d'un llenguatge en concret amb el vídeo
- `youtube-dl -f "bestaudio" -c -w -i -x --audio-format mp3 -o "%(title)s.%(ext)s" 'url_to_playlist'` : descarrega una playlist i extreu-ne els MP3s

## Links
- More information: https://rg3.github.io/youtube-dl/
