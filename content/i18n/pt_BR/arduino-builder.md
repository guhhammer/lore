# arduino-builder

Uma ferramenta de linha de comando para compilar sketches do arduino. AVIDO DE OBSOLESCÊNCIA: Esta ferramenta está sendo descontinuada e substituida pelo `arduino`.

## Usage
- `arduino-builder -compile caminho/para/sketch.ino` : compila um sketch
- `arduino-builder -debug-level nivel` : define o nível de debug (1 a 10, o padrão é 5)
- `arduino-builder -build-path caminho/para/diretorio` : define um diretório de compilação customizado
- `arduino-builder -build-options-file caminho/para/build.options.json` : usa um arquivo com as opções de compilação, em vez de especificar `-hardware`, `-tools`, etc. manualmente toda hora
- `arduino-builder -verbose true` : habilita o modo verboso

## Links
- More information: https://github.com/arduino/arduino-builder
