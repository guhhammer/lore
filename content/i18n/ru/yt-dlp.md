# yt-dlp

Форк youtube-dl с дополнительными возможностями и исправлениями. Загружать видео с YouTube и других сайтов.

## Usage
- `yt-dlp "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : скачать видео или плейлист (с параметрами по умолчанию)
- `yt-dlp -F "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : вывести список доступных для скачивания форматов видео
- `yt-dlp -f "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4]" "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : скачать видео или плейлист в лучшем доступном формате MP4 (по умолчанию "bv\*+ba/b")
- `yt-dlp -x "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : извлечь аудио из видео (требуется ffmpeg или ffprobe)
- `yt-dlp -x --audio-format mp3 --audio-quality 0 "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : указать формат и качество извлекаемого аудио (от 0 (лучшее) до 10 (худшее), по умолчанию 5)
- `yt-dlp -I 2,4:6,-1 "https://youtube.com/playlist?list=PLbzoR-pLrL6pTJfLQ3UwtB-3V4fimdqnA"` : скачать только второй, с четвёртого по шестой и последний элементы плейлиста (нумерация начинается с 1, а не с 0)
- `yt-dlp -o "%(uploader)s/%(playlist)s/%(playlist_index)s - %(title)s.%(ext)s" "https://www.youtube.com/user/TheLinuxFoundation/playlists"` : скачать все плейлисты канала/пользователя YouTube, сохраняя каждый плейлист в отдельный каталог
- `yt-dlp -u пользователь -p пароль -P "путь/к/каталогу" -o "%(playlist)s/%(chapter_number)s - %(chapter)s/%(title)s.%(ext)s" "https://www.udemy.com/java-tutorial"` : скачать курс Udemy, сохраняя каждую главу в отдельный каталог

## Links
- More information: https://github.com/yt-dlp/yt-dlp#usage-and-options
