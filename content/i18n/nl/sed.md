# sed

Pas tekst aan op een scriptbare manier.

## Usage
- `commando | sed 's/apple/mango/g'` : vervang alle `apple` (basis `regex`) met `mango` (basis `regex`) in alle invoerregels en toon het resultaat in `stdout`
- `commando | sed -f pad/naar/script.sed` : voer een specifiek scriptbestand (f) uit en toon het resultaat in `stdout`
- `commando | sed -n '1p'` : toon alleen de eerste regel in `stdout`

## Links
- More information: https://manned.org/sed.1posix
