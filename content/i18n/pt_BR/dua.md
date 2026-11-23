# dua

Dua (Analisador de Uso de Disco) é uma ferramenta para análise conveniente do uso de disco dado um diretório.

## Usage
- `dua caminho/para/diretorio` : analisa um diretório específico
- `dua --apparent-size` : exibe o tamanho aparente ao invés do uso do disco
- `dua --count-hard-links` : contabiliza arquivos hard-linked cada vez que eles são encontrados
- `dua aggregate` : agrega o espaço em disco consumido de um ou mais diretórios ou arquivos
- `dua interactive` : inicia a interface de usuário
- `dua --format metric|binary|bytes|GB|GiB|MB|MiB` : seleciona o formato para contagem de bytes
- `dua --threads numero` : escolhe o número de threads a serem usadas

## Links
- More information: https://github.com/Byron/dua-cli
