# rename-item

PowerShell commando om een item te hernoemen.

## Usage
- `Rename-Item -Path "pad\naar\bestand" -NewName "nieuwe_bestandsnaam"` : hernoem een bestand
- `Rename-Item -Path "pad\naar\map" -NewName "nieuwe_mapnaam"` : hernoem een map
- `Rename-Item -Path "pad\naar\bestand" -NewName "pad\naar\nieuwe_bestandsnaam"` : hernoem en verplaats een bestand
- `Rename-Item -Path "pad\naar\bestand" -NewName "nieuwe_bestandsnaam" -Force` : hernoem een bestand geforceerd
- `Rename-Item -Path "pad\naar\bestand" -NewName "nieuwe_bestandsnaam" -Confirm` : vraag om bevestiging voordat een bestand wordt hernoemd
