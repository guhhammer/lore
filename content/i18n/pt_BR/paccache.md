# paccache

Um utilitário de limpeza do cache do `pacman`.

## Usage
- `paccache -r` : remove tudo, exceto as 3 versões mais recentes do cache do `pacman`
- `paccache -rk num_versoes` : define o número de versões do pacote para manter
- `paccache -d` : executa um teste e mostra o número de pacotes candidatos para exclusão
- `paccache -m caminho/para/diretorio` : move os pacotes candidatos para um diretório ao invés de excluí-los

## Links
- More information: https://manned.org/paccache
