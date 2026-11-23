# screenrecord

Neem een video op van een mobiel scherm. Opmerking: dit commando kan alleen worden gebruikt via `adb shell`.

## Usage
- `screenrecord pad/naar/bestand.mp4` : neem het scherm op
- `screenrecord --size 1280x720 pad/naar/bestand.mp4` : neem het scherm op met een specifieke resolutie
- `screenrecord --bit-rate 6000000 pad/naar/bestand.mp4` : neem het scherm op met een specifieke bitrate
- `screenrecord --time-limit 180 pad/naar/bestand.mp4` : neem het scherm op met een maximale duur (in seconden, 180 seconden is het maximum)
- `screenrecord --help` : toon de help

## Links
- More information: https://developer.android.com/tools/adb#screenrecord
