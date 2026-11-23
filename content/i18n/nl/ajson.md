# ajson

Voert JSONPath uit op JSON-objecten.

## Usage
- `ajson '$..json[?(@.path)]' pad/naar/bestand.json` : lees JSON uit een bestand en voer een opgegeven JSONPath-expressie uit
- `cat pad/naar/bestand.json | ajson '$..json[?(@.path)]'` : lees JSON van `stdin` en voer een gespecificeerde JSONPath-expressie uit
- `ajson 'avg($..price)' 'https://example.com/api/'` : lees JSON van een URL en evalueer een opgegeven JSONPath-expressie
- `echo '3' | ajson '2 * pi * $'` : lees wat eenvoudige JSON en bereken een waarde

## Links
- More information: https://github.com/spyzhov/ajson#console-application
