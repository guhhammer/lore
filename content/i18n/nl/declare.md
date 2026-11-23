# declare

Declareer variabelen en geef ze attributen.

## Usage
- `declare variabele="waarde"` : declareer een string variabele met de gespecificeerde waarde
- `declare -i variabele="waarde"` : declareer een integer variabele met de gespecificeerde waarde
- `declare -a variabele=(item_a item_b item_c)` : declareer een array variabele met de gespecificeerde waarde
- `declare -A variabele=([sleutel_a]=item_a [sleutel_b]=item_b [sleutel_c]=item_c)` : declareer een associatieve array variabele met de gespecificeerde waarde
- `declare -r variabele="waarde"` : declareer een alleen-lezen string variabele met de gespecificeerde waarde
- `declare -g variabele="waarde"` : declareer een globale variabele binnen een functie met de gespecificeerde waarde
- `declare -f functie_naam` : print een functie-definitie
- `declare -p variabele_naam` : print een variabele-definitie

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-declare
