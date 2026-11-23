# prctl

Pobieraj lub ustawiaj kontrolę zasobów uruchomionych procesów, zadań i projektów.

## Usage
- `prctl pid` : sprawdź limity procesów i uprawnienia
- `prctl -P pid` : sprawdź limity procesów i uprawnienia w formacie przetwarzalnym przez maszynę
- `prctl -n process.max-file-descriptor pid` : uzyskaj określony limit dla uruchomionego procesu

## Links
- More information: https://www.unix.com/man-page/sunos/1/prctl
