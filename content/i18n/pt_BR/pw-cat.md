# pw-cat

Toca e grava arquivos de áudio através do PipeWire.

## Usage
- `pw-cat -p caminho/para/arquivo.wav` : toca um arquivo WAV no alvo padrão
- `pw-cat -q 0..15 -p caminho/para/arquivo.wav` : toca um arquivo WAV com uma qualidade de reamostragem específica (4 por padrão)
- `pw-cat -r --volume 1.25 caminho/para/arquivo.wav` : faz uma gravação com o volume em 125%
- `pw-cat -r --rate 6000 caminho/para/arquivo.wav` : faz uma gravação com uma taxa de amostragem diferente
- `pw-cat -h` : exibe ajuda

## Links
- More information: https://docs.pipewire.org/page_man_pw-cat_1.html
