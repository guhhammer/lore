# crane push

Stuur lokale image-inhoud naar een externe registry.

## Usage
- `crane push pad/naar/tarball image_naam` : stuur lokale image naar externe registry
- `crane push pad/naar/tarball image_naam --image-refs pad/naar/bestand` : schrijf een lijst van gepubliceerde image-referenties naar een bestand
- `crane push pad/naar/tarball image_naam --index` : stuur een verzameling images als een enkele index (vereist als pad meerdere images heeft)
- `crane push -h` : toon de help
