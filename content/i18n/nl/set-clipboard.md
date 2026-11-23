# set-clipboard

PowerShell commando om inhoud naar het klembord te kopiëren.

## Usage
- `Set-Clipboard -Value "tekst"` : kopieer tekst naar het klembord
- `Set-Clipboard -Value @("tekst 1", "tekst 2", "tekst 3")` : kopieer meerdere teksten naar het klembord, gescheiden door een nieuwe regel
- `Set-Clipboard -Path "pad\naar\bestanden_of_mappen"` : kopieer bestanden of mappen naar het klembord
- `Set-Clipboard -Path "pad\naar\bestand1","pad\naar\bestand2","pad\naar\bestand3"` : kopieer meerdere bestanden
- `Set-Clipboard ""` : wis het klembord
