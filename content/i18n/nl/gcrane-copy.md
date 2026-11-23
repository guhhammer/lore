# gcrane copy

Kopieer efficiënt een remote image van bron naar doel terwijl de digest-waarde behouden blijft.

## Usage
- `gcrane cp bron doel` : kopieer een image van bron naar doel
- `gcrane copy bron doel -j aantal_kopieën` : stel het maximale aantal gelijktijdige kopieën in, standaard is 20
- `gcrane copy bron doel -r` : doorzoek repositories recursief
- `gcrane copy -h` : toon de help
