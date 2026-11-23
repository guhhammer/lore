# cat

Afișează și concatenează fișiere.

## Usage
- `cat cale/către/fișier` : afișează conținutul unui fișier la `stdout`
- `cat cale/către/fișier1 cale/către/fișier2 ... >cale/către/fișier_de_ieșire` : concatenează mai multe fișiere într-un fișier de ieșire
- `cat cale/către/fișier1 cale/către/fișier2 ... >>cale/către/fișier_de_ieșire` : adaugă mai multe fișiere la un fișier de ieșire
- `cat -u /dev/tty12 >/dev/tty13` : copiază conținutul unui fișier într-un fișier de ieșire fără memorie tampon
- `cat - >cale/către/fișier` : scrie `stdin` într-un fișier

## Links
- More information: https://manned.org/cat.1posix
