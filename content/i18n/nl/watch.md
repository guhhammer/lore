# watch

Voer een programma periodiek uit en bekijk de uitvoer in volledig schermmodus.

## Usage
- `watch commando` : voer een commando herhaaldelijk uit en toon het resultaat
- `watch -n 60 commando` : voer een commando elke 60 seconden opnieuw uit
- `watch -d df` : controleer de schijfruimte en markeer deze verschillen zodra ze zich voordoen
- `watch "commando1 | commando2 | commando3"` : voer een pipeline herhaaldelijk uit en toon het resultaat
- `watch -g lsblk` : sluit `watch` af als de zichtbare uitvoer verandert
- `watch -c ls --color=always` : interpreteer terminalbesturingstekens

## Links
- More information: https://manned.org/watch
