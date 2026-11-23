# scc

Utilitário escrito em GO que conta linhas de código.

## Usage
- `scc` : mostra o número de linhas de código no diretório atual
- `scc caminho/para/diretorio` : mostra o número de linhas de código de um diretório especificado
- `scc --by-file` : mostra o número de linhas de código por arquivo
- `scc -f tabular|wide|json|csv|cloc-yaml|html|html-table` : mostra o resultado usando um formato específico (formato padrão é o `tabular`)
- `scc -i go, java, js` : conta apenas os arquivos com as extensões especificadas
- `scc --exclude-dir .git,.hg` : exclui diretórios da contagem
- `scc -s files|name|lines|blanks|code|comments|complexity` : mostra output organizado de acordo com o parâmetro especificado (organização padrão é `files`)
- `scc -h` : mostra ajuda

## Links
- More information: https://github.com/boyter/scc#usage
