# yt-dlp

Um fork do youtube-dl com novas funções e correções. Baixe videos do YouTube e outros sites.

## Usage
- `yt-dlp "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : baixa um video ou playlist (com as opções padrão do comando abaixo)
- `yt-dlp -F "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : lista os formatos disponíveis para download de um vídeo
- `yt-dlp -f "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4]" "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : baixa um vídeo ou playlist usando o melhor vídeo MP4 disponível (o padrão é "bv\*+ba/b")
- `yt-dlp -x "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : extrai o áudio de um vídeo (requer ffmpeg ou ffprobe)
- `yt-dlp -x --audio-format mp3 --audio-quality 0 "https://www.youtube.com/watch?v=oHg5SJYRHA0"` : especifica o formato e a qualidade do áudio extraido (entre 0 (maior qualidade) e 10 (menor qualidade), padrão = 5)
- `yt-dlp -I 2,4:6,-1 "https://youtube.com/playlist?list=PLbzoR-pLrL6pTJfLQ3UwtB-3V4fimdqnA"` : baixa apenas o segundo, quarto, quinto, sexto e último item de uma playlist (o primeiro item é 1, e não 0)
- `yt-dlp -o "%(uploader)s/%(playlist)s/%(playlist_index)s - %(title)s.%(ext)s" "https://www.youtube.com/user/TheLinuxFoundation/playlists"` : baixa todas as playlists de um canal/usuário do YouTube mantendo cada playlist em um diretório separado
- `yt-dlp -u usuário -p senha -P "caminho/para/diretório" -o "%(playlist)s/%(chapter_number)s - %(chapter)s/%(title)s.%(ext)s" "https://www.udemy.com/java-tutorial"` : baixa um curso da Udemy mantendo cada capítulo em um diretório separado

## Links
- More information: https://github.com/yt-dlp/yt-dlp#usage-and-options
