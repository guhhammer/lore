# ffmpeg

Ferramenta de conversão de vídeo.

## Usage
- `ffmpeg -i caminho/para/vídeo.mp4 -vn caminho/para/som.mp3` : extrai o som de um vídeo e salva-o como MP3
- `ffmpeg -i caminho/para/audio_de_entrada.flac -ar 44100 -sample_fmt s16 caminho/para/audio_de_saida.wav` : transcodifica um arquivo FLAC to formato de CD Red Book (44100Hz, 16bit)
- `ffmpeg -i caminho/para/vídeo.mp4 -vf 'scale=-1:1000' -r 15 caminho/para/saída.gif` : salva um vídeo como GIF, escalando a altura para 1000px e definindo a taxa de quadros para 15
- `ffmpeg -f image2 -i caminho/para/quadro_%d.jpg vídeo.mpg|vídeo.gif` : combina imagens numeradas (`quadro_1.jpg`, `quadro_2.jpg`, etc) em um vídeo ou GIF
- `ffmpeg -i caminho/para/vídeo_entrada.mp4 -ss mm:ss -to mm2:ss2 -c copy caminho/para/vídeo_saída.mp4` : corta um vídeo de um dado tempo inicial mm:ss até um tempo final mm2:ss2 (omita a opção -to para cortar o vídeo até o final)
- `ffmpeg -i caminho/para/vídeo_entrada.avi -c:a aac -b:a 128k -c:v libx264 -crf 23 caminho/para/vídeo_saída.mp4` : converte um vídeo AVI para MP4. AAC Áudio @ 128kbit, h264 Vídeo @ CRF 23
- `ffmpeg -i caminho/para/vídeo_entrada.mkv -c copy caminho/para/vídeo_saída.mp4` : remixa um vídeo MKV para MP4 sem recodificar o áudio ou o vídeo
- `ffmpeg -i caminho/para/vídeo_entrada.mp4 -c:v libvpx-vp9 -crf 30 -b:v 0 -c:a libopus -vbr on -threads número_de_threads caminho/para/vídeo_saída.webm` : converte um vídeo MP4 para o codec VP9. Para a melhor qualidade, use um valor CRF (faixa recomendada 15-35) e -b:v DEVE ser 0

## Links
- More information: https://ffmpeg.org/ffmpeg.html#Options
