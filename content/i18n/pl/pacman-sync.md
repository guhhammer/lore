# pacman-sync

Narzędzie do zarządzania pakietami w Arch Linuksie.

## Usage
- `sudo pacman --sync nazwa_pakietu` : zainstaluj nowy pakiet
- `sudo pacman --sync --refresh --sysupgrade` : zsynchronizuj i zaktualizuj wszystkie pakiety (użyj `--downloadonly` aby pobrać pakiety i ich nie zaktualizować)
- `sudo pacman --sync --refresh --sysupgrade --noconfirm nazwa_pakietu` : zaktualizuj wszystkie pakiety i zainstaluj nowy bez pytania
- `pacman --sync --search "zapytanie"` : przeszukaj bazę danych pakietów używając wyrażenia regularnego lub słowa klucz
- `pacman --sync --info nazwa_pakietu` : wyświetl informacje o pakiecie
- `sudo pacman --sync --refresh --sysupgrade --overwrite ścieżka/do/pliku` : nadpisz pliki będące w konflikcie podczas aktualizacji pakietów
- `sudo pacman --sync --refresh --sysupgrade --ignore nazwa_pakietu` : zsynchronizuj i zaktualizuj wszystkie pakiety, ale zignoruj konkretny pakiet (można użyć więcej niż raz)
- `sudo pacman --sync --clean` : usuń niezainstalowane pakiety i nieużywane repozytoria z pamięci podręcznej (użyj dwa razy opcji `--clean`, aby wyczyścić wszystkie pakiety)

## Links
- More information: https://manned.org/pacman.8
