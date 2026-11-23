# pacman-mirrors

Wygeneruj listę serwerów lustrzanych dla Manjaro Linuksa. Każde uruchomienie pacman-mirrors wymaga zsynchronizowanej bazy danych oraz zaktualizowania systemu używając `sudo pacman -Syyu`.

## Usage
- `sudo pacman-mirrors --fasttrack` : wygeneruj listę serwerów lustrzanych używając domyślnych ustawień
- `pacman-mirrors --status` : wyświetl status aktualnych serwerów lustrzanych
- `pacman-mirrors --get-branch` : pokaż aktualną gałąź
- `sudo pacman-mirrors --api --set-branch stable|unstable|testing` : przęłącz na inną gałąź
- `sudo pacman-mirrors --geoip` : wygeneruj listę serwerów lustrzanych, używając tylko tych w twoim kraju

## Links
- More information: https://wiki.manjaro.org/index.php?title=Pacman-mirrors
