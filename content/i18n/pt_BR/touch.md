# touch

Cria arquivos e define tempo de acesso/modificação.

## Usage
- `touch caminho/para/arquivo1 caminho/para/arquivo2 ...` : cria arquivos especificados
- `touch -c -a|m caminho/para/arquivo1 caminho/para/arquivo2 ...` : define o tempo de acesso ou modificação do arquivo como o atual e não cria o arquivo se ele não existir
- `touch -c -t YYYYMMDDHHMM.SS caminho/para/arquivo1 caminho/para/arquivo2 ...` : define o tempo do arquivo para um valor especificado e não cria o arquivo se ele não existir
- `touch -c -r ~/.emacs caminho/para/arquivo1 caminho/para/arquivo2 ...` : define o tempo de um arquivo específico para o tempo de outro arquivo e não cria o arquivo se ele não existir
