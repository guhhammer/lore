# bash

Bourne-Again SHell, `sh`-ondersteunende commandoregel-interpreteerder.

## Usage
- `bash` : start een interactieve shellsessie
- `bash --norc` : start een interactieve shellsessie zonder het laden van startup configuratie
- `bash -c "echo 'bash is executed'"` : voer een commando uit
- `bash pad/naar/script.sh` : voer commando's van bestand uit
- `bash -x pad/naar/script.sh` : voer commando's van bestand uit (x), en print elk commando voordat het wordt uitgevoerd
- `bash -e pad/naar/script.sh` : voer commando's van bestand uit, en stop bij de eerste fout
- `echo "echo 'bash is executed'" | bash` : voer commando's van `stdin` uit
- `bash -r` : start een beperkte shellsessie

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Invoking-Bash
