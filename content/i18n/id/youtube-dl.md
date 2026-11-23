# youtube-dl

Unduh video dari YouTube dan situs web lain.

## Usage
- `youtube-dl 'https://www.youtube.com/watch?v=oHg5SJYRHA0'` : unduh suatu video atau daftar putar
- `youtube-dl -F 'https://www.youtube.com/watch?v=Mwa0_nE9H7A'` : tampilkan daftar format yang tersedia untuk video atau daftar putar
- `youtube-dl -f "best[height<=480]" 'https://www.youtube.com/watch?v=oHg5SJYRHA0'` : unduh video atau daftar putar dengan kualitas tertentu
- `youtube-dl -x --audio-format mp3 'url'` : unduh audio dari suatu video dan ubah menjadi berkas MP3
- `youtube-dl -f bestvideo+bestaudio 'url'` : unduh gabungan video dan audio dengan kualitas terbaik
- `youtube-dl -f mp4 -o "%(playlist_index)s-%(title)s oleh %(uploader)s pada %(upload_date)s di dalam %(playlist)s.%(ext)s" 'url'` : unduh satu atau beberapa video sebagai berkas MP4 dengan nama tertentu
- `youtube-dl --sub-lang en --write-sub 'https://www.youtube.com/watch?v=Mwa0_nE9H7A'` : unduh video bersama dengan subtitle bahasa tertentu
- `youtube-dl -f "bestaudio" -c -w -i -x --audio-format mp3 -o "%(title)s.%(ext)s" 'url_to_playlist'` : mengunduh daftar putar dan ekstrak MP3 darinya

## Links
- More information: https://rg3.github.io/youtube-dl/
