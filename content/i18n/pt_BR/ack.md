# ack

Uma ferramenta de pesquisa similar ao `grep`, otimizada para programadores.

## Usage
- `ack "padrão_de_busca"` : procura por arquivos que contenham o termo, ou a expressão regular, no diretório atual
- `ack -i "padrão_de_busca"` : procura um padrão sem diferenciar maiúsculas e minúsculas
- `ack -o "padrão_de_busca"` : procura por linhas correspondentes ao padrão, imprimindo apenas o texto correspondente e não o resto da linha
- `ack -t ruby "padrão_de_busca"` : limita a busca a um tipo específico de arquivo
- `ack -t noruby "padrão_de_busca"` : não busca arquivos de um tipo específico
- `ack -c -h "padrão_de_busca"` : conta o total de correspondências encontradas
- `ack -c -l "padrão_de_busca"` : imprime o nome dos arquivos e o número de correspondências para cada arquivo
- `ack --help-types` : lista todos os valores que podem ser utilizados com `--type`

## Links
- More information: https://beyondgrep.com/documentation/
