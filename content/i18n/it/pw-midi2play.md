# pw-midi2play

Riproduce clip MIDI 2.0 tramite PipeWire. Nota: Questo comando è un alias per `pw-cat --playback --midi-clip`.

## Usage
- `pw-midi2play percorso/del/file.mid` : riproduce un file clip MIDI 2.0 sulla destinazione predefinita
- `pw-midi2play -M midi|ump percorso/del/file.mid` : riproduce un clip MIDI, forzando il formato MIDI da usare (predefinito: `ump`)
- `pw-midi2play -h` : mostra l'aiuto

## Links
- More information: https://docs.pipewire.org/page_man_pw-cat_1.html
