# move-item

Verplaats of hernoem bestanden, mappen, registersleutels en andere PowerShell-gegevensitems. Dit commando kan alleen worden uitgevoerd onder PowerShell.

## Usage
- `Move-Item pad\naar\bron pad\naar\doel` : hernoem een bestand of map wanneer het doelwit geen bestaande map is
- `Move-Item pad\naar\bron pad\naar\bestaande_map` : verplaats een bestand of map naar een bestaande map
- `Move-Item -LiteralPath "pad\naar\bron" pad\naar\bestand_of_map` : hernoem of verplaats bestand(en) met een specifieke naam (behandel geen speciale karakters in strings)
- `Move-Item pad\naar\bron1 , pad\naar\bron2 ... pad\naar\bestaande_map` : verplaats meerdere bestanden naar een bestaande map, waardoor de bestandsnamen ongewijzigd blijven
- `Move-Item pad\naar\bron_sleutel1 , pad\naar\bron_sleutel2 ... pad\naar\nieuwe_of_bestaande_sleutel` : verplaats of hernoem registersleutel(s)
- `mv -Force pad\naar\bron pad\naar\doel` : vraag niet om bevestiging voordat bestaande bestanden of registersleutels worden overschreven
- `mv -Confirm pad\naar\bron pad\naar\doel` : vraag om bevestiging voordat bestaande bestanden worden overschreven, ongeacht bestandsrechten
- `mv -WhatIf pad\naar\bron pad\naar\doel` : verplaats bestanden in de dry-run-modus en toon bestanden en mappen die kunnen worden verplaatst zonder ze uit te voeren
