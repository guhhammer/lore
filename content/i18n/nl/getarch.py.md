# getarch.py

Bepaal de OS-architectuur (x86 of x64) van een remote Windows-systeem. Onderdeel van de Impacket-suite.

## Usage
- `getArch.py -target doel` : controleer de architectuur van een enkel doelsysteem
- `getArch.py -targets pad/naar/doelen_bestand` : controleer de architectuur van meerdere doelen van een bestand (één per regel)
- `getArch.py -target doel -timeout seconden` : stel een custom socket timeout in (standaard is 2 seconden)
- `getArch.py -target doel -debug` : schakel debug-modus in voor gedetailleerde uitvoer

## Links
- More information: https://github.com/fortra/impacket
