# unexpand

Converteer spaties naar tabs.

## Usage
- `unexpand pad/naar/bestand` : converteer spaties in elk bestand naar tabs en schrijf naar `stdout`
- `unexpand` : converteer spaties naar tabs en lees van `stdin`
- `unexpand -a pad/naar/bestand` : converteer alle spaties, in plaats van alleen de voorloopspaties
- `unexpand --first-only pad/naar/bestand` : converteer alleen leidende reeksen van spaties (overschrijft -a)
- `unexpand -t nummer pad/naar/bestand` : plaats tabs een bepaald aantal tekens uit elkaar, niet 8 (activeert -a)
