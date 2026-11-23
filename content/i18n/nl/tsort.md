# tsort

Voer een topologische sortering uit. Een veelvoorkomend gebruik is om de afhankelijkheidsvolgorde van knooppunten in een gerichte acyclische grafiek te tonen.

## Usage
- `tsort pad/naar/bestand` : voer een topologische sortering uit consistent met een gedeeltelijke sortering per regel van invoer gescheiden door spaties
- `echo -e "UI Backend\nBackend Database\nDocs UI" | tsort` : voer een topologische sortering uit consistent op strings
