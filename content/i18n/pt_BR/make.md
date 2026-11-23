# make

Ferramenta de execução de tarefas para os destinos descritos no Makefile. Principalmente utilizada para controlar a compilação de um executável a partir do código-fonte.

## Usage
- `make` : executa o primeiro destino especificado no Makefile (geralmente chamado de "all")
- `make destino` : executa um destino específico
- `make -j 4 destino` : executa um destino específico, executando 4 tarefas simultaneamente em paralelo
- `make -f caminho/para/arquivo` : usa um Makefile específico
- `make -C caminho/para/diretorio` : executa o make a partir de outro diretório
- `make -B destino` : força a execução de um destino, mesmo que os arquivos de origem não tenham sido alterados
- `make destino variavel=novo_valor` : substitui uma variável definida no Makefile
- `make -e destino` : substitui variáveis definidas no Makefile pelo ambiente

## Links
- More information: https://www.gnu.org/software/make/manual/make.html
