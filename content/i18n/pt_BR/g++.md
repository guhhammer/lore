# g++

Compila arquivos de código fonte C++. Parte do GCC (GNU Compiler Collection).

## Usage
- `g++ caminho/para/fonte1.cpp caminho/para/fonte1.cpp ... -o caminho/para/executável_de_saída` : compila um arquivo de código fonte para um binário executável
- `g++ caminho/para/fonte.css -Wall -o executável_de_saída` : ativa saída de todos os erros e avisos
- `g++ caminho/para/fonte.cpp -Wall -g -Og -o caminho/para/executável_de_saída` : mostra avisos comuns, símbolos de depuração na saída, e otimiza sem afetar a depuração
- `g++ caminho/para/fonte.cpp -std=c++98|c++11|c++14|c++17 -o caminho/para/executável_de_saída` : escolhe um padrão de linguagem para o qual compilar (C++98/C++11/C++14/C++17)
- `g++ caminho/para/fonte.cpp -o caminho/para/executável_de_saída -Icaminho/para/cabeçalho -Lcaminho/para/biblioteca -lnome_da_biblioteca` : inclui bibliotecas localizadas em um caminho diferente do arquivo de código fonte
- `g++ -c caminho/para/fonte1.cpp caminho/para/fonte2.cpp ... && g++ -o caminho/para/executável_de_saída caminho/para/fonte1.o caminho/para/fonte2.o ...` : compila e vincula múltiplos arquivos de código fonte em um binário executável
- `g++ caminho/para/fonte.cpp -O1|2|3|fast -o caminho/para/executavel_de_saída` : otimiza o programa compilado para desempenho
- `g++ --version` : mostra versão

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/C_002b_002b-Dialect-Options.html
