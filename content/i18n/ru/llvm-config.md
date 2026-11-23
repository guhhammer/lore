# llvm-config

Получать различную конфигурационную информацию, необходимую для компиляции программ, использующих LLVM. Обычно вызывается из систем сборки, например в Makefile или скриптах конфигурации.

## Usage
- `clang++ $(llvm-config --cxxflags --ldflags --libs) --output путь/к/исполняемому_файлу путь/к/исходному_файлу.cc` : скомпилировать и скомпоновать программу на основе LLVM
- `llvm-config --prefix` : вывести `PREFIX` установки LLVM
- `llvm-config --targets-built` : вывести все целевые платформы, поддерживаемые сборкой LLVM

## Links
- More information: https://llvm.org/docs/CommandGuide/llvm-config.html
