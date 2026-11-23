# invoke-item

Open bestanden in hun bijbehorende standaardprogramma's. Opmerking: dit commando kan alleen gebruikt worden via PowerShell.

## Usage
- `Invoke-Item -Path pad\naar\bestand` : open een bestand in het standaardprogramma
- `Invoke-Item -Path pad\naar\map\*` : open alle bestanden in een map
- `Invoke-Item -Path pad\naar\map\*.png` : open alle PNG's in een map
- `Invoke-Item -Path pad\naar\map\* -Include *trefwoord*` : open alle bestanden in een map die een specifiek trefwoord bevatten
- `Invoke-Item -Path pad\naar\map\* -Exclude *trefwoord*` : open alle bestanden in een map, behalve die een specifiek trefwoord bevatten
- `Invoke-Item -Path pad\naar\map\* -WhatIf` : simuleer een uitvoering om te bepalen welke bestanden in een map geopend zullen worden via `Invoke-Item`
