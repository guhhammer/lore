# llvm-bcanalyzer

Анализатор LLVM Bitcode (`.bc`).

## Usage
- `llvm-bcanalyzer путь/к/файлу.bc` : вывести статистику о файле Bitcode
- `llvm-bcanalyzer -dump путь/к/файлу.bc` : вывести представление в формате SGML и статистику о файле Bitcode
- `cat путь/к/файлу.bc | llvm-bcanalyzer` : прочитать файл Bitcode из `stdin` и проанализировать его

## Links
- More information: https://llvm.org/docs/CommandGuide/llvm-bcanalyzer.html
