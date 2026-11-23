# touch

Atualizar as timestamps de um ficheiro para a hora atual. Se o ficheiro não existir, cria um ficheiro vazio, a menos que seja passado o parâmetro -c ou -h.

## Usage
- `touch caminho/para/ficheiro1 caminho/para/ficheiro2 ...` : cria um novo ficheiro vazio, ou atualizar as timestamps para a hora atual
- `touch -t YYYYMMDDHHMM.SS caminho/para/ficheiro1 caminho/para/ficheiro2 ...` : define as timestamps de um ficheiro para a hora especificada
