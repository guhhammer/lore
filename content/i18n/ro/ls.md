# ls

Afișează conținutul unui folder.

## Usage
- `ls -1` : afișează fișierele câte unul pe linie
- `ls -a` : afișează toate fișierele, inclusiv cele ascunse
- `ls -F` : afișează fișierele cu un simbol la final pentru a indica tipul (folder/, link_simbolic@, executabil*, ...)
- `ls -la` : afișează toate fișierele în format lung (permisiuni, proprietar, dimensiune și data modificării)
- `ls -lh` : afișează fișierele în format lung cu dimensiunile în unități citibile (KiB, MiB, GiB)
- `ls -lSR` : afișează fișierele în format lung, sortate descrescător după dimensiune (Size), recursiv
- `ls -ltr` : afișează fișierele în format lung, sortate după timpul modificării și în ordine inversă (cele mai vechi primele)
- `ls -d */` : afișează doar folderele

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/ls-invocation.html
