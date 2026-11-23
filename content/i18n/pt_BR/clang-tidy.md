# clang-tidy

Um linter C/C++ baseado em LLVM para encontrar violações de estilo, bugs e falhas de segurança por meio de análise estática.

## Usage
- `clang-tidy caminho/para/arquivo.cpp` : executa verificações padrão em um arquivo fonte
- `clang-tidy caminho/para/arquivo.cpp -checks=-*,cppcoreguidelines-*` : não executa quaisquer verificações além da verificação `cppcoreguidelines` em um arquivo
- `clang-tidy -checks=* -list-checks` : lista todas as verificações disponíveis
- `clang-tidy caminho/para/arquivo.cpp -- -Imeu_projeto/include -Ddefinições` : especifica defines (definições) e includes (inclusões) como opções de compilações (após `--`)

## Links
- More information: https://clang.llvm.org/extra/clang-tidy/
