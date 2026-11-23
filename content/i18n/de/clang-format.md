# clang-format

Programm zum Auto-Formatieren von C/C++/Java/JavaScript/Objective-C/Protobuf/C#-Code.

## Usage
- `clang-format pfad/zu/quelldatei.cpp` : formatiere eine Datei und schreibe das Ergebnis nach `stdout`
- `clang-format -i pfad/zu/quelldatei.cpp` : überschreibe eine Datei mit ihrem formatierten Inhalt
- `clang-format --style LLVM|GNU|Google|Chromium|Microsoft|Mozilla|WebKit pfad/zu/quelldatei.cpp` : formatiere eine Datei mit einem bestimmten Code-Stil
- `clang-format --style file pfad/zu/quelldatei.cpp` : formatiere eine Datei mit der `.clang-format`-Datei aus einem der Überverzeichnisse der Quelldatei
- `clang-format --style LLVM|GNU|Google|Chromium|Microsoft|Mozilla|WebKit --dump-config >.clang-format` : generiere eine eigene `.clang-format`-Datei

## Links
- More information: https://clang.llvm.org/docs/ClangFormat.html
