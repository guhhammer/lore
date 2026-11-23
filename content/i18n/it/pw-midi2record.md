# pw-midi2record

Registra clip MIDI 2.0 tramite PipeWire. Nota: Questo comando è un alias per `pw-cat --record --midi-clip`.

## Usage
- `pw-midi2record percorso/del/file.mid` : registra un file clip MIDI 2.0 dalla sorgente predefinita
- `pw-midi2record -M midi|ump percorso/del/file.mid` : registra un clip MIDI, forzando il formato MIDI da usare (predefinito: `ump`)
- `pw-midi2record -h` : mostra l'aiuto

## Links
- More information: https://docs.pipewire.org/page_man_pw-cat_1.html
