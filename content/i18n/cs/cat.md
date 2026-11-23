# cat

Vypisuje a spojuje soubory.

## Usage
- `cat cesta/k/souboru` : vypíše obsah souboru do `stdout`
- `cat cesta/k/souboru1 cesta/k/souboru2 ... >cesta/k/vystupnimu_souboru` : spojí několik souborů do výstupního souboru
- `cat cesta/k/souboru1 cesta/k/souboru2 ... >>cesta/k/vystupnimu_souboru` : přidá několik souborů do výstupního souboru
- `cat -u /dev/tty12 >/dev/tty13` : zkopíruje obsah souboru do výstupního souboru bez vyrovnávací paměti
- `cat - >cesta/k/souboru` : vypíše `stdin` do souboru

## Links
- More information: https://manned.org/cat.1posix
