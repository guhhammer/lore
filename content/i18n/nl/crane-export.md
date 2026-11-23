# crane export

Exporteer het bestandssysteem van een containerimage als een tarball.

## Usage
- `crane export image_naam -` : schrijf de tarball naar `stdout`
- `crane export image_naam pad/naar/tarball` : schrijf de tarball naar een bestand
- `crane export - pad/naar/bestand` : lees de image vanuit `stdin`
