# youtube-dl

Pobieraj wideo i audio z YouTube i podobnych portali.

## Usage
- `youtube-dl 'https://www.youtube.com/watch?v=oHg5SJYRHA0'` : pobierz plik wideo lub wszystkie pliki z playlisty
- `youtube-dl -F 'https://www.youtube.com/watch?v=Mwa0_nE9H7A'` : wypisz wszystkie formaty dostępne dla filmu lub playlisty
- `youtube-dl -f "best[height<=480]" 'https://www.youtube.com/watch?v=oHg5SJYRHA0'` : pobierz wideo lub playlistę w wybranej jakości
- `youtube-dl -x --audio-format mp3 'url'` : pobierz audio z wideo w formacie mp3
- `youtube-dl -f bestvideo+bestaudio 'url'` : pobierz wideo ze ścieżką audio złączone w jednym pliku w najlepszej dostępnej jakości
- `youtube-dl -f mp4 -o "%(playlist_index)s-%(title)s by %(uploader)s on %(upload_date)s in %(playlist)s.%(ext)s" 'url'` : pobierz wideo jako pliki MP4 i nazwij wedle schematu
- `youtube-dl --sub-lang en --write-sub 'https://www.youtube.com/watch?v=Mwa0_nE9H7A'` : pobierz plik razem z napisami
- `youtube-dl -f "bestaudio" -c -w -i -x --audio-format mp3 -o "%(title)s.%(ext)s" 'adres_url_playlisty'` : pobierz playlistę i wyodrębnij z niej pliki MP3

## Links
- More information: https://rg3.github.io/youtube-dl/
