# crane pull

Haal externe images op via referentie en sla hun inhoud lokaal op.

## Usage
- `crane pull image_naam pad/naar/tarball` : haal externe image op
- `crane pull image_naam pad/naar/tarball --annotate-ref` : bewaar de image-referentie die is gebruikt om op te halen als een annotatie wanneer gebruikt met `--format=oci`
- `crane pull image_naam pad/naar/tarball -c pad/naar/cache` : cache image-lagen in een specifieke map
- `crane pull image_naam pad/naar/tarball -format format_naam` : specificeer het formaat waarin images moeten worden opgeslagen (standaard `tarball`)
- `crane pull -h` : toon de help
