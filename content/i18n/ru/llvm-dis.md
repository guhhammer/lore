# llvm-dis

Конвертировать файлы LLVM Bitcode в читаемое промежуточное представление LLVM (IR).

## Usage
- `llvm-dis путь/к/входному_файлу.bc -o -` : конвертировать файл Bitcode в LLVM IR и записать результат в `stdout`
- `llvm-dis путь/к/файлу.bc` : конвертировать файл Bitcode в файл LLVM IR с тем же именем
- `llvm-dis путь/к/входному_файлу.bc -o путь/к/выходному_файлу.ll` : конвертировать файл Bitcode в LLVM IR, записав результат в указанный файл

## Links
- More information: https://www.llvm.org/docs/CommandGuide/llvm-dis.html
