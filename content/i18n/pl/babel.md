# babel

Transpiler, który konwertuje kod ze składni JavaScript ES6/ES7 na składnię ES5.

## Usage
- `babel ścieżka/do/pliku` : transpiluj określony plik wejściowy i wypisz dane wyjściowe do `stdout`
- `babel ścieżka/do/pliku_wejściowego --out-file ścieżka/do/pliku_wyjściowego` : transpiluj określony plik wejściowy i zapisz wyjście do określonego pliku
- `babel ścieżka/do/pliku_wejściowego --watch` : transpiluj plik wejściowy przy każdej zmianie
- `babel ścieżka/do/katalogu_wejściowego` : transpiluj cały katalog plików
- `babel ścieżka/do/katalogu_wejściowego --ignore ignorowany_plik1,ignorowany_plik2,...` : zignoruj określone pliki oddzielone przecinkami w katalogu
- `babel ścieżka/do/pliku_wejściowego --minified` : transpiluj i wypisz jako zminimalizowany JavaScript
- `babel ścieżka/do/pliku_wejściowego --presets preset1,preset2,...` : wybierz zestaw ustawień dla formatowania wyjściowego
- `babel --help` : wyświetl pomoc

## Links
- More information: https://babeljs.io/docs/babel-cli
