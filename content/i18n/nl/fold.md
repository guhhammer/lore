# fold

Breek elke regel in een invoerbestand af om in een gespecificeerde breedte te passen en toon het in `stdout`.

## Usage
- `fold pad/naar/bestand` : breek elke regel af op de standaard breedte (80 tekens)
- `fold -w30 pad/naar/bestand` : breek elke regel af op een breedte van "30"
- `fold -w5 -s pad/naar/bestand` : breek elke regel af op een breedte van "5" en breek de regel bij spaties (zet elk door spaties gescheiden woord op een nieuwe regel, woorden langer dan 5 tekens worden afgebroken)

## Links
- More information: https://manned.org/fold.1p
