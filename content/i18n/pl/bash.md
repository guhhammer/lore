# bash

Bourne-Again SHell, interpreter komend powłoki systemowej kompatybilny z `sh`.

## Usage
- `bash` : rozpocznij interaktywną sesję powłoki
- `bash --norc` : rozpocznij interaktywną sesję powłoki bez ładowania konfiguracji
- `bash -c "echo 'bash jest uruchomiony'"` : wywołaj określone komendy
- `bash ścieżka/do/skryptu.sh` : uruchom podany skrypt
- `bash -x ścieżka/do/skryptu.sh` : wykonaj podany skrypt, wypisując wszystkie komendy przed ich wykonaniem
- `bash -e ścieżka/do/skryptu.sh` : wykonaj podany skrypt do wystąpienia pierwszego błędu
- `echo "echo 'bash jest uruchomiony'" | bash` : wykonaj komendy ze `stdin`
- `bash -r` : uruchom sesję w trybie restrykcyjnym

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Invoking-Bash
