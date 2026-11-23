# env

Toon de omgeving of voer een programma uit in een aangepaste omgeving.

## Usage
- `env` : toon de omgeving
- `env programma` : voer een programma uit. Meestal gebruikt in scripts na de shebang (#!) voor het opzoeken van het pad naar het programma
- `env -i programma` : wis de omgeving en voer een programma uit
- `env -u variabele programma` : verwijder een variabele van de omgeving en voer een programma uit
- `env variabele=waarde programma` : zet een variabele en voer een programma uit
- `env variabele1=waarde variabele2=waarde variabele3=waarde ... programma` : zet meerdere variabelen en voer een programma uit
- `env -a aangepaste_naam programma` : voer een programma uit onder een andere naam

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/env-invocation.html
