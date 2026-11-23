# pw-cat

Riproduce e registra file audio tramite PipeWire.

## Usage
- `pw-cat -p percorso/del/file.wav` : riproduce un file WAV sulla destinazione predefinita
- `pw-cat -pm percorso/del/file.mid` : riproduce un file MIDI
- `pw-cat -pd percorso/del/file.dsf` : riproduce un file DSD
- `pw-cat -po percorso/del/file.ac3` : riproduce un file audio compresso usando passthrough (richiede integrazione FFmpeg)
- `pw-cat -p -q 0..15 percorso/del/file.wav` : riproduce un file WAV con una qualità di ricampionamento specificata (4 predefinita)
- `pw-cat -rm percorso/del/file.mid` : registra un file MIDI
- `pw-cat -r --volume 1.25 percorso/del/file.wav` : registra un campione a un livello di volume del 125%
- `pw-cat -r --rate 6000 percorso/del/file.wav` : registra un campione usando una frequenza di campionamento diversa

## Links
- More information: https://docs.pipewire.org/page_man_pw-cat_1.html
