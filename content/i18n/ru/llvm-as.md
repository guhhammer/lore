# llvm-as

Ассемблер промежуточного представления LLVM (`.ll`) в Bitcode (`.bc`).

## Usage
- `llvm-as -o путь/к/выходному_файлу.bc путь/к/исходному_файлу.ll` : ассемблировать IR-файл
- `llvm-as --module-hash -o путь/к/выходному_файлу.bc путь/к/исходному_файлу.ll` : ассемблировать IR-файл и включить хеш модуля в созданный файл Bitcode
- `cat путь/к/исходному_файлу.ll | llvm-as -o путь/к/выходному_файлу.bc` : прочитать IR-файл из `stdin` и ассемблировать его

## Links
- More information: https://llvm.org/docs/CommandGuide/llvm-as.html
