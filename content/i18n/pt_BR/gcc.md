# gcc

Pré-processa e compila arquivos de código fonte C e C++, depois monta-os e vincula-os.

## Usage
- `gcc caminho/para/arquivo_fonte1.c caminho/para/arquivo_fonte2.c ... -o caminho/para/executável_de_saída` : compila múltiplos arquivos de código fonte, produzindo um arquivo executável
- `gcc caminho/para/fonte.c -Wall -o executável_de_saída` : ativa a saída de todos os erros e avisos
- `gcc caminho/para/fonte.c -Wall -g -Og -o caminho/para/executável_de_saída` : mostra avisos comuns, símbolos de depuração na saída, e otimiza sem afetar a depuração
- `gcc caminho/para/fonte.c -o caminho/para/executável_de_saída -Icaminho/para/header -Lcaminho/para/biblioteca -lnome_biblioteca` : inclui bibliotecas de um local diferente
- `gcc -S caminho/para/fonte.c` : compila o código fonte para instruções Assembler
- `gcc -c caminho/para/font.c` : compila o código fonte sem efetuar vinculação
- `gcc caminho/para/fonte.c -O1|2|3|fast -o caminho/para/executável_de_saída` : otimiza o programa compilado para desempenho
- `gcc --version` : mostra versão

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/
