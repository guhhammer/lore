# clang-tidy

Ein LLVM-basierter C/C++ Linter zum Finden von Stilverletzungen, Bugs und Sicherheitsmängeln durch statische Codeanalyse.

## Usage
- `clang-tidy pfad/zu/quelldatei.cpp` : führe die Standard-Checks für eine Quelldatei aus
- `clang-tidy pfad/zu/quelldatei.cpp -checks=-*,cppcoreguidelines-*` : prüfe nur ob eine Datei den `cppcoreguidelines` Checks entspricht
- `clang-tidy -checks=* -list-checks` : liste alle verfügbaren Checks auf
- `clang-tidy pfad/zu/quelldatei.cpp -- -Imein_projekt/include -Ddefinitions` : lege defines und includes als Kompilierungsoptionen fest (nach `--`)

## Links
- More information: https://clang.llvm.org/extra/clang-tidy/
