# addr2line

Converte endereços de um binário em nomes de arquivos e números de linha.

## Usage
- `addr2line -e caminho/do/executavel endereco` : exibe o nome do arquivo e o número da linha do código-fonte de um endereço de instrução de um executável
- `addr2line -e caminho/do/executavel -f endereco` : exibe o nome da função, nome do arquivo e número da linha
- `addr2line -e caminho/do/executavel -f -C endereco` : desembaraça o nome da função em código C++

## Links
- More information: https://manned.org/addr2line
