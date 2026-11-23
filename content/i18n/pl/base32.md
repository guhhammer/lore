# base32

Enkoduj lub dekoduj plik lub standardowe wejście do/z Base32, na standardowe wyjście.

## Usage
- `base32 ścieżka/do/pliku` : enkoduj plik
- `base32 -w 0|76|... ścieżka/do/pliku` : zawijaj zakodowane wyjście do określonej szerokości (`0` wyłącza zawijanie)
- `base32 -d ścieżka/do/pliku` : dekoduj plik
- `jakiespolecenie | base32` : enkoduj z `stdin`
- `jakiespolecenie | base32 -d` : dekoduj z `stdin`

## Links
- More information: https://manned.org/base32
