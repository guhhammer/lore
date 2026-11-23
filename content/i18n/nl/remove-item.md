# remove-item

Verwijder bestanden, mappen, evenals registersleutels en subsleutels. Deze opdracht kan alleen door PowerShell worden uitgevoerd.

## Usage
- `Remove-Item pad\naar\bestand_of_sleutel1 , pad\naar\bestand_of_sleutel2 ...` : verwijder specifieke bestanden of registersleutels (zonder subsleutels)
- `Remove-Item -Force pad\naar\bestand1 , pad\naar\bestand2 ...` : verwijder verborgen of alleen-lezen bestanden
- `Remove-Item -Confirm pad\naar\bestand_of_sleutel1 , pad\naar\bestand_of_sleutel2 ...` : verwijder specifieke bestanden of registersleutels, waarbij interactief om bevestiging wordt gevraagd vóór elke verwijdering
- `Remove-Item -Recurse pad\naar\bestand_of_map1 , pad\naar\bestand_of_map2 ...` : verwijder specifieke bestanden en mappen recursief (Windows 10 versie 1909 of hoger)
- `Remove-Item -Recurse pad\naar\sleutel1 , pad\naar\sleutel2 ...` : verwijder specifieke Windows-registersleutels en al hun subsleutels
- `Remove-Item -WhatIf pad\naar\bestand1 , pad\naar\bestand2 ...` : voer een dry-run van het verwijderproces uit
