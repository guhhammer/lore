# tail

Toon het laatste deel van een bestand.

## Usage
- `tail pad/naar/bestand` : toon de laatste 10 regels van een bestand
- `tail pad/naar/bestand1 pad/naar/bestand2 ...` : toon de laatste 10 regels van meerdere bestanden
- `tail -5 pad/naar/bestand` : toon laatste 5 regels in een bestand
- `tail -n +aantal pad/naar/bestand` : toon een bestand vanaf een specifiek regelnummer
- `tail -c aantal pad/naar/bestand` : toon een specifiek aantal bytes vanaf het einde van een opgegeven bestand
- `tail -f pad/naar/bestand` : toon de laatste regels van een bestand en blijf het bestand lezen tot `<Ctrl c>`
- `tail -F pad/naar/bestand` : blijf het bestand lezen tot `<Ctrl c>`, ook als het bestand niet toegankelijk is
- `tail -n aantal -s seconden -f pad/naar/bestand` : toon de laatste `aantal` regels in een bestand en ververs iedere `seconden` seconden

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tail-invocation.html
