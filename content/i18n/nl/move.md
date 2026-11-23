# move

Verplaats of hernoem bestanden en mappen. In PowerShell is dit commando een alias van `Move-Item`. Deze documentatie is gebaseerd op de Command Prompt (`cmd`) versie van `move`.

## Usage
- `tldr move-item` : bekijk de documentatie van het equivalente PowerShell commando
- `move pad\naar\bron pad\naar\doel` : hernoem een bestand of map als het doel een niet-bestaande map is
- `move pad\naar\bron pad\naar\bestaande_map` : verplaats een bestand of map naar een bestaande map
- `move C:\pad\naar\bron D:\pad\naar\doel` : verplaats een bestand of map naar een andere schijf
- `move /Y pad\naar\bron pad\naar\bestaande_map` : vraag niet om bevestiging voordat bestaande bestanden worden overschreven
- `move /-Y pad\naar\bron pad\naar\bestaande_map` : vraag om bevestiging voordat bestaande bestanden worden overschreven, ongeacht de bestandspermissies
