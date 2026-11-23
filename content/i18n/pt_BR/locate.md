# locate

Encontre nomes de arquivos rapidamente.

## Usage
- `locate padrão` : procura por padrões no banco de dados. Nota: o banco de dados é recalculado periodicamente (geralmente semanalmente ou diariamente)
- `locate '*/nome_do_arquivo'` : procura um arquivo pelo seu nome de arquivo exato(um padrão que não contém caracteres curingas é interpretado como `*pattern*`)
- `sudo updatedb` : recalcula o banco de dados. Você precisa fazer se você quiser achar os arquivos recentementes adicionados

## Links
- More information: https://manned.org/locate
