# pssh

Programa de SSH paralelo.

## Usage
- `pssh -i -H "servidor1 servidor2" hostname -i` : executa um comando em dois servidores e imprime a saída em cada servidor em linha
- `pssh -H servidor1 -H servidor2 -o caminho/para/diretório_de_saída hostname -i` : executa um comando e salva a saída em arquivos separados
- `pssh -i -h caminho/para/arquivo_de_servidores hostname -i` : executa um comando em vários servidores, especificados em um arquivo separado por nova linha
- `pssh -i -h caminho/para/arquivo_de_servidores -A -l nome_de_usuário_do_root hostname -i` : executa um comando como root (isso solicitará a senha do root)
- `pssh -i -h caminho/para/arquivo_de_servidores -x "-O VisualHostKey=yes" hostname -i` : executa um comando com argumentos SSH adicionais
- `pssh -i -h caminho/para/arquivo_de_servidores -p 10 'cd dir; ./script.sh; exit'` : executa um comando limitando o número de conexões paralelas para 10

## Links
- More information: https://manned.org/pssh
