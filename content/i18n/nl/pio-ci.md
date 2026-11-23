# pio ci

Bouw PlatformIO projects met een arbitraire broncodestructuur. Dit zal een nieuw tijdelijk project maken waarnaartoe de broncode gekopieerd zal worden.

## Usage
- `pio ci pad/naar/project` : bouw een PlatformIO project in de standaard tijdelijke systeemmap en verwijder het naderhand
- `pio ci -l pad/naar/bibliotheek_map pad/naar/project` : bouw een PlatformIO project en specificeer specifieke bibliotheken
- `pio ci -b board pad/naar/project` : bouw een PlatformIO project en specificeer een specifiek board (`pio boards` toont ze allemaal)
- `pio ci --build-dir pad/naar/bouw_map pad/naar/project` : bouw een PlatformIO project in een specifieke map
- `pio ci --keep-build-dir pad/naar/project` : bouw een PlatformIO project en verwijder de bouwmap niet
- `pio ci -c pad/naar/platformio.ini` : bouw een PlatformIO project met een specifiek configuratiebestand

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/cmd_ci.html
