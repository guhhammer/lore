# wc

Tel regels, woorden en bytes.

## Usage
- `wc -l pad/naar/bestand` : tel alle regels in een bestand
- `wc -w pad/naar/bestand` : tel alle woorden in een bestand
- `wc -c pad/naar/bestand` : tel alle bytes in een bestand
- `wc -m pad/naar/bestand` : tel alle karakters in een bestand (rekening houdend met multi-byte karakters)
- `find . | wc` : tel alle regels, woorden en bytes van `stdin`
- `wc -L pad/naar/bestand` : tel de lengte van de langste regel in aantal karakters

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/wc-invocation.html
