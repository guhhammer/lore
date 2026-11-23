# cat

Skriv ut og slå sammen filer.

## Usage
- `cat sti/til/fil` : skriv ut innholdet i en fil til `stdout`
- `cat sti/til/fil1 sti/til/fil2 ... >sti/til/utdatafil` : slå sammen flere filer til en utdatafil
- `cat sti/til/fil1 sti/til/fil2 ... >>sti/til/utdatafil` : legg flere filer til på slutten av en utdatafil
- `cat -u /dev/tty12 >/dev/tty13` : kopier innholdet i en fil til en utdatafil uten bufring
- `cat - >sti/til/fil` : skriv `stdin` til en fil

## Links
- More information: https://manned.org/cat.1posix
