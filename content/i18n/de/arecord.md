# arecord

Sound Recorder für den ALSA-Soundkarten-Treiber.

## Usage
- `arecord -vv --format=cd pfad/zu/datei.wav` : nehme einen Schnipsel in CD-Qualität auf (beende die Aufnahme mit `<Ctrl c>`)
- `arecord -vv --format=cd --duration=10 pfad/zu/datei.wav` : nehme einen Schnipsel in CD-Qualität auf mit einer festen Länge von 10 Sekunden
- `arecord -vv --format=cd --file-type raw | lame -r - pfad/zu/datei.mp3` : nehme einen Schnipsel auf und speichere es als MP3 (beende die Aufnahme mit `<Ctrl c>`)
- `arecord --list-devices` : liste alle Soundkarten und digitalen Ausgabe Geräte
- `arecord --interactive` : benutze das interaktive Interface (z.B. `<Space>` oder `<Enter>` für Play oder Pause)

## Links
- More information: https://manned.org/arecord
