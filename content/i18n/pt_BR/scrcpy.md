# scrcpy

Exiba e controle seu dispositivo Android em um desktop.

## Usage
- `scrcpy` : espelha a tela de um dispositivo conectado
- `scrcpy -S -w` : desliga a tela do dispositivo e previne que ele entre em modo de suspensão enquanto espelha a tela
- `scrcpy -s 0123456789abcdef|192.168.0.1:5555` : espelha a tela de um dispositivo com base no seu ID ou endereço de IP (visível no comando `adb devices`)
- `scrcpy -t` : mostra toques no dispositivo físico
- `scrcpy -r caminho/para/arquivo.mp4` : grava a tela do dispositivo
- `scrcpy --push-target caminho/para/diretório` : especifica um diretório alvo para enviar arquivos para o dispositivo (exceto APKs)
- `scrcpy --video-source camera` : visualiza a câmera do dispositivo (requer Android 12 ou superior)
- `scrcpy --video-source camera --camera-size 1920x1080 --v4l2-sink /dev/video0 --no-playback` : cria um dispositivo Video4Linux2 a partir da câmera do dispositivo (o pacote `v4l2loopback` deve estar instalado)

## Links
- More information: https://github.com/Genymobile/scrcpy
