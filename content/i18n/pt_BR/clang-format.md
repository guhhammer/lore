# clang-format

Formata automaticamente código C/C++/Java/JavaScript/Objective-C/Protobuf/C#.

## Usage
- `clang-format caminho/para/arquivo` : formata um arquivo e exibe o resultado para a `stdout` (saída padrão)
- `clang-format -i caminho/para/arquivo` : formata um arquivo "in-place", ou seja, salvando nele mesmo
- `clang-format --style LLVM|GNU|Google|Chromium|Microsoft|Mozilla|WebKit caminho/para/arquivo` : formata um arquivo usando um estilo de código predefinido
- `clang-format --style file caminho/para/arquivo` : formata um arquivo usando o arquivo `.clang-format` em um dos diretórios pais do arquivo fonte
- `clang-format --style LLVM|GNU|Google|Chromium|Microsoft|Mozilla|WebKit --dump-config >.clang-format` : gera um arquivo `.clang-format` personalizado

## Links
- More information: https://clang.llvm.org/docs/ClangFormat.html
