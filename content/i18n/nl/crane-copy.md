# crane copy

Kopieer efficiënt een remote image van bron naar doel terwijl de digest-waarde behouden blijft.

## Usage
- `crane cp bron doel` : kopieer een image van bron naar doel
- `crane cp bron doel -a` : kopieer alle tags
- `crane cp bron doel -j int` : stel het maximumaantal gelijktijdige kopieën in, standaard is GOMAXPROCS
- `crane cp bron doel -n` : voorkom het overschrijven van bestaande tags in het doel
- `crane cp -h` : toon de help
