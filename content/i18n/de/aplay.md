# aplay

Command-line Musik Player für den ALSA-Soundkarten-Treiber.

## Usage
- `aplay pfad/zu/datei` : spiele eine bestimmte Datei (Abtastrate, Bittiefe, etc. werden automatisch für das Dateiformat erkannt)
- `aplay --duration=10 --rate=2500 pfad/zu/datei` : spiele die ersten 10 Sekunden einer bestimmten Datei mit 2500 Hz
- `aplay --channels=1 --file-type raw --rate=22050 --format=mu_law pfad/zu/datei` : spiele die rohe Datei mit 22050 Hz, mono, 8-bit, als Mu-Lw `.au` Datei

## Links
- More information: https://manned.org/aplay
