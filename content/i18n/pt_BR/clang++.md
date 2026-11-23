# clang++

Compila arquivos fonte escritos em C++. Parte de LLVM.

## Usage
- `clang++ caminho/para/arquivo_fonte1.cpp caminho/para/arquivo_fonte2.cpp ... -o caminho/para/executável_resultante` : compila um conjunto de arquivos do código-fonte para um executável binário
- `clang++ caminho/para/arquivo_fonte.cpp -Wall -o executável_resultante` : ativa a saída de todos os erros e avisos
- `clang++ caminho/para/arquivo_fonte.cpp -Wall -g -Og -o caminho/para/executável_resultante` : mostra avisos comuns, faz depuração de símbolos na saída e otimiza sem afetar a depuração
- `clang++ caminho/para/arquivo_fonte.cpp -std=c++20 -o caminho/para/executável_resultante` : escolhe um padrão da linguagem para o qual deve-se compilar
- `clang++ caminho/para/arquivo_fonte.cpp -o caminho/para/executável_resultante -Icaminho/para/cabeçalho -Lcaminho/para/biblioteca -lcaminho/para/nome_da_biblioteca` : inclui bibliotecas localizadas em um caminho diferente do arquivo fonte
- `clang++ -S -emit-llvm caminho/para/arquivo_fonte.cpp -o caminho/para/saída.ll` : compila o código-fonte na Representação Intermediária (IR) do LLVM
- `clang++ caminho/para/arquivo_fonte.cpp -O1|2|3|fast -o caminho/para/executável_resultante` : otimiza o programa compilado para desempenho
- `clang++ --version` : mostra versão

## Links
- More information: https://clang.llvm.org/docs/UsersManual.html#command-line-options
