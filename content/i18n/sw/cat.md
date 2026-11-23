# cat

Onyesha na unganisha faili.

## Usage
- `cat njia/ya/faili` : onyesha maudhui ya faili kwenye `stdout`
- `cat njia/ya/faili1 njia/ya/faili2 ... >njia/ya/faili_la_matokeo` : unganisha faili kadhaa kuwa faili moja la matokeo
- `cat njia/ya/faili1 njia/ya/faili2 ... >>njia/ya/faili_la_matokeo` : ongeza faili kadhaa mwishoni mwa faili la matokeo
- `cat -u /dev/tty12 >/dev/tty13` : nakili maudhui ya faili kwenye faili la matokeo bila buffering
- `cat - >njia/ya/faili` : andika `stdin` kwenye faili

## Links
- More information: https://manned.org/cat.1posix
