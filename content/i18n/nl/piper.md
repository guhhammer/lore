# piper

Een snel, lokaal neuraal tekst-naar-spraaksysteem.

## Usage
- `echo Te zeggen tekst | piper -m pad/naar/model.onnx -f uitvoerbestand.wav` : genereer een WAV-bestand (f) met een tekst-naar-spraak-model (uitgaande van een configuratiebestand op model_pad + `.json`)
- `echo Te zeggen tekst | piper -m pad/naar/model.onnx -c pad/naar/model.onnx.json -f uitvoerbestand.wav` : genereer een WAV-bestand (f) met een model en specificeer het bijbehorende JSON-configuratiebestand
- `echo Warum? | piper -m de_DE-thorsten_emotional-medium.onnx --speaker 1 -f angry.wav` : selecteer een specifieke spreker in een stem met meerdere sprekers door het ID-nummer van de spreker op te geven
- `echo Hello world | piper -m en_GB-northern_english_male-medium.onnx --output-raw -f - | mpv -` : stream de uitvoer naar de mpv-mediaspeler
- `echo Speaking twice the speed. With added drama! | piper -m file.onnx --length_scale 0.5 --sentence_silence 2 -f drama.wav` : spreek twee keer zo snel, met grote pauzes tussen zinnen

## Links
- More information: https://rhasspy.github.io/piper-samples
