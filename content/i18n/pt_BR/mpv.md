# mpv

Um tocador de vídeo/audio baseado no MPlayer.

## Usage
- `mpv url|caminho/para/arquivo` : toca um vídeo ou áudio de uma URL ou arquivo
- `<ArrowLeft>|<ArrowRight>` : avança/retrocede 5 segundos
- `<ArrowDown>|<ArrowUp>` : avança/retrocede 1 minuto
- `<[>|<]>` : diminui ou aumenta a velocidade de reprodução em 10%
- `<s>` : captura a imagem do quadro atual (salva em `./mpv-shotNNNN.jpg` por padrão)
- `mpv --speed 0.01..100 caminho/para/arquivo` : toca um arquivo em uma velocidade especificada (1 por padrão)
- `mpv --profile nome_do_perfil caminho/para/arquivo` : toca um arquivo usando um perfil definido no arquivo `mpv.conf`
- `mpv /dev/video0` : mostra a saída da webcam ou de outro dispositivo de entrada de vídeo

## Links
- More information: https://mpv.io/manual/stable/
