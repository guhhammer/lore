# pw-cat

Speel en neem audio-bestanden op via PipeWire.

## Usage
- `pw-cat -p pad/naar/bestand.wav` : speel een WAV-bestand af over de standaard target
- `pw-cat -pm pad/naar/bestand.mid` : speel een MIDI-bestand af
- `pw-cat -pd pad/naar/bestand.dsf` : speel een DSD-bestand af
- `pw-cat -po pad/naar/bestand.ac3` : speel een gecomprimeerd audiobestand af met passthrough (vereist FFmpeg-integratie)
- `pw-cat -p -q 0..15 pad/naar/bestand.wav` : speel een WAV-bestand af met een specifieke resamplerkwaliteit (standaard 4)
- `pw-cat -rm pad/naar/bestand.mid` : neem een MIDI-bestand op
- `pw-cat -r --volume 1.25 pad/naar/bestand.wav` : neem een voorbeeldopname op met een volume van 125%
- `pw-cat -r --rate 6000 pad/naar/bestand.wav` : neem een voorbeeldopname op met een andere sample rate

## Links
- More information: https://docs.pipewire.org/page_man_pw-cat_1.html
