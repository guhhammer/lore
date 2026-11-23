# yt-dlp

Een youtube-dl fork met extra functies en fixes. Download video's van YouTube en andere websites.

## Usage
- `yt-dlp "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : download een video of afspeellijst (met de standaardopties van onderstaand commando)
- `yt-dlp -F "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : toon de beschikbare downloadbare formaten voor een video
- `yt-dlp -f "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4]" "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : download een video of afspeellijst met de best beschikbare MP4-video (standaard is "bv\*+ba/b")
- `yt-dlp -x "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : extraheer audio uit een video (vereist ffmpeg of ffprobe)
- `yt-dlp -x --audio-format mp3 --audio-quality 0 "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : specificeer het audioformaat en de audiokwaliteit van de geëxtraheerde audio (tussen 0 (beste) en 10 (slechtste), standaard = 5)
- `yt-dlp -I 2,4:6,-1 "https://youtube.com/playlist?list=PLbzoR-pLrL6pTJfLQ3UwtB-3V4fimdqnA"` : download alleen het tweede, vierde, vijfde, zesde en laatste item in een afspeellijst (het eerste item is 1, niet 0)
- `yt-dlp -o "%(uploader)s/%(playlist)s/%(playlist_index)s - %(title)s.%(ext)s" "https://www.youtube.com/user/TheLinuxFoundation/playlists"` : download alle afspeellijsten van een YouTube-kanaal/gebruiker, waarbij elke afspeellijst in een aparte map wordt bewaard
- `yt-dlp -u gebruiker -p wachtwoord -P "pad/naar/map" -o "%(playlist)s/%(chapter_number)s - %(chapter)s/%(title)s.%(ext)s" "https://www.udemy.com/java-tutorial"` : download een Udemy-cursus, waarbij elk hoofdstuk in een aparte map wordt bewaard

## Links
- More information: https://github.com/yt-dlp/yt-dlp#usage-and-options
