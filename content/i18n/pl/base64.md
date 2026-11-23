# base64

Enkoduj lub dekoduj plik lub standardowe wejście do/z Base64, na standardowe wyjście.

## Usage
- `base64 ścieżka/do/pliku` : enkoduj plik
- `base64 -w 0|76|... ścieżka/do/pliku` : zawijaj zakodowane wyjście na określonej szerokości (`0` wyłącza zawijanie)
- `base64 -d ścieżka/do/pliku` : dekoduj plik
- `jakiespolecenie | base64` : enkoduj z `stdin`
- `jakiespolecenie | base64 -d` : dekoduj z `stdin`

## Links
- More information: https://manned.org/base64
