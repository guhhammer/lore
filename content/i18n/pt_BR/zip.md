# zip

Empacota e comprime de arquivos em arquivos zip.

## Usage
- `zip -r caminho/para/comprimido.zip caminho/para/arquivo_ou_diretorio1 caminho/para/arquivo_ou_diretorio2 ...` : adiciona arquivos/diretórios a um arquivo zip específico
- `zip -d caminho/para/comprimido.zip caminho/para/arquivo_ou_diretorio1 caminho/para/arquivo_ou_diretorio2 ...` : remove arquivos de um arquivo zip
- `zip -r caminho/para/comprimido.zip caminho/para/arquivo_ou_diretorio1 caminho/para/arquivo_ou_diretorio2 ... -x caminho/a/ser/excluido` : empacota arquivos/diretórios excluindo arquivos específicos
- `zip -r -0-9 caminho/para/comprimido.zip caminho/para/arquivo_ou_diretorio1 caminho/para/arquivo_ou_diretorio2 ...` : empacota e compacta arquivos com um nível de compressão específico (0 - o mais baixo, 9 - o mais alto)
- `zip -re caminho/para/comprimido.zip caminho/para/arquivo_ou_diretorio1 caminho/para/arquivo_ou_diretorio2 ...` : cria um zip encriptado com uma senha específica
- `zip -rs 3g caminho/para/comprimido.zip caminho/para/arquivo_ou_diretorio1 caminho/para/arquivo_ou_diretorio2 ...` : empacota arquivos/diretórios para um zip dividido em múltiplas partes (ex.: partes de 3 GB)
- `zip -sf caminho/para/comprimido.zip` : mostra o conteúdo de um zip específico

## Links
- More information: https://manned.org/zip
