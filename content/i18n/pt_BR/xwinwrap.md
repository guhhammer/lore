# xwinwrap

Usa um reprodutor de vídeo ou um programa como plano de fundo.

## Usage
- `xwinwrap -b -nf -ov -- mpv --wid wid --loop --no-audio --no-resume-playback --panscan=1.0 caminho/para/video.mp4` : reproduz um vídeo usando mpv
- `xwinwrap -b -nf -fs -ov -- mpv --wid wid --loop --no-audio --no-resume-playback --panscan=1.0 caminho/para/video.mp4` : reproduz um vídeo em tela cheia usando mpv
- `xwinwrap -b -nf -ov -o 0.8 --- mpv --wid wid --loop --no-audio --no-resume-playback --panscan=1.0 caminho/para/video.mp4` : reproduz um vídeo usando mpv com 80% de opacidade
- `xwinwrap -g 1600x900+1920 -b -nf -ov -- mpv --wid wid --loop --no-audio --no-resume-playback --panscan=1.0 caminho/para/video.mkv` : reproduz um vídeo usando mpv em um segundo monitor 1600x900 com 1920 de distância do eixo X

## Links
- More information: https://github.com/r00tdaemon/xwinwrap
