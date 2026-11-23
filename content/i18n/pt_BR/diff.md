# diff

Compara diretórios e arquivos.

## Usage
- `diff arquivo_antigo arquivo_novo` : compara arquivos (mostra as mudanças necessárias para transformar `arquivo_antigo` em `arquivo_novo`)
- `diff -w arquivo_antigo arquivo_novo` : compara arquivos, ignorando espaço
- `diff -y arquivo_antigo arquivo_novo` : compara arquivos, mostrando diferenças lado a lado
- `diff -u arquivo_antigo arquivo_novo` : compara arquivos, mostrando as diferenças de forma padronizada como feito por `git diff`
- `diff -r arquivo_antigo arquivo_novo` : compara diretórios recursivamente (mostra nomes de diretórios e arquivos diferentes assim como mudanças nos arquivos)
- `diff -r -q arquivo_antigo arquivo_novo` : compara diretórios, mostrando apenas os nomes dos arquivos diferentes
- `diff -a -u -N arquivo_antigo arquivo_novo >diferenca.patch` : cria um arquivo patch para o Git a partir das diferenças entre dois arquivos, tratando arquivos ausentes como vazios
- `diff -d --color=always arquivo_antigo arquivo_novo` : compara arquivos, mostra a saída em cores e tenta fortemente encontrar um conjunto menor de alterações

## Links
- More information: https://manned.org/diff
