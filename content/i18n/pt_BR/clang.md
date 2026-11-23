# clang

Compila arquivos fonte escritos em C, C++ e Objective-C. Pode ser usado como um substituto drop-in para GCC. Parte de LLVM.

## Usage
- `clang caminho/para/arquivo_fonte1.c caminho/para/arquivo_fonte2.c ... -o caminho/para/executável_resultante` : compila vários arquivos fonte para um executável
- `clang caminho/para/arquivo_fonte.c -Wall -o executável_resultante` : ativa a saída de todos os erros e avisos
- `clang caminho/para/arquivo_fonte.c -Wall -g -Og -o caminho/para/executável_resultante` : mostra avisos comuns, faz depuração de símbolos na saída e otimiza sem afetar a depuração
- `clang caminho/para/arquivo_fonte.c -o caminho/para/executável_resultante -Icaminho/para/cabeçalho -Lcaminho/para/biblioteca -lnome_da_biblioteca` : inclui bibliotecas de um caminho diferente
- `clang -S -emit-llvm caminho/para/arquivo_fonte.c -o path/to/output.ll` : compila o código-fonte na Representação Intermediária (IR) do LLVM
- `clang -c caminho/para/arquivo_fonte.c` : compila o código-fonte em um arquivo objeto sem vincular
- `clang caminho/para/arquivo_fonte.c -O1|2|3|fast -o caminho/para/executável_resultante` : otimiza o programa compilado para desempenho
- `clang --version` : mostra versão

## Links
- More information: https://clang.llvm.org/docs/ClangCommandLineReference.html
