# apt-file

Wyszukaj pliki w pakietach APT, w tym jeszcze nie zainstalowanych.

## Usage
- `sudo apt update` : zaktualizuj bazę metadanych
- `apt-file find część/ścieżki/do/pliku` : wyszukaj pakiet, który zawiera określony plik lub ścieżkę
- `apt-file list pakiet` : wyświetl zawartośċ określonego pakietu
- `apt-file find -x wyrażenie_regularne` : wyszukaj pakiety, które pasują do podanego `wyrażenia_regularnego`

## Links
- More information: https://manned.org/apt-file
