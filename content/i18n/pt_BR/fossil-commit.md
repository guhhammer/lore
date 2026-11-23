# fossil commit

Faz commit de arquivos para um repositório Fossil.

## Usage
- `fossil ci` : cria uma nova versão contendo todas as alterações no checkout atual; o usuário será solicitado a inserir um comentário
- `fossil ci -m "comentário"` : cria uma nova versão contendo todas as alterações no checkout atual, usando o comentário especificado
- `fossil ci -M caminho/para/arquivos_de_mensagem_de_commit` : cria uma nova versão contendo todas as alterações no checkout atual com um comentário lido de um arquivo específico
- `fossil ci caminho/para/arquivo1 caminho/para/arquivo2 ...` : cria uma nova versão contendo alterações dos arquivos especificados; o usuário será solicitado a fornecer um comentário

## Links
- More information: https://fossil-scm.org/home/help/commit
