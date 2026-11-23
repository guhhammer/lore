# youtube-dl

Scarica video da YouTube ed altri siti web.

## Usage
- `youtube-dl 'https://www.youtube.com/watch?v=oHg5SJYRHA0'` : scarica un video od una playlist
- `youtube-dl -F 'https://www.youtube.com/watch?v=Mwa0_nE9H7A'` : elenca tutti i formati in cui un video od una playlist sono disponibili
- `youtube-dl -f "best[height<=480]" 'https://www.youtube.com/watch?v=oHg5SJYRHA0'` : scarica un video od una playlist con la qualità specificata
- `youtube-dl -x --audio-format mp3 'url'` : scarica l'audio di un video e lo converte in file MP3
- `youtube-dl -f bestvideo+bestaudio 'url'` : scarica l'audio di migliore qualità e il video di migliore qualità e li unisce
- `youtube-dl -f mp4 -o "%(title)s di %(uploader)s del %(upload_date)s in %(playlist)s.%(ext)s" 'url'` : scarica una playlist di video e li salva come file MP4 dai nomi personalizzati
- `youtube-dl --sub-lang it --write-sub 'https://www.youtube.com/watch?v=Mwa0_nE9H7A'` : scarica, assieme al video, i sottotitoli in una lingua specificata
- `youtube-dl -f "bestaudio" -c -w -i -x --audio-format mp3 -o "%(title)s.%(ext)s" 'url_della_playlist'` : scarica una playlist, ne estrae l'audio e lo salva in formato mp3

## Links
- More information: https://rg3.github.io/youtube-dl/
