# dot_clean

Mescla ._* arquivos com arquivos nativos correspondentes.

## Usage
- `dot_clean caminho/para/diretório` : mescla todos os `._*` arquivos recursivamente
- `dot_clean -f caminho/para/diretório` : não mescla recursivamente todos `._*` em um diretório (flat merge)
- `dot_clean -m caminho/para/diretório` : mescla e exclui todos os arquivos `._*`
- `dot_clean -n caminho/para/diretório` : somente exclui arquivos `._*` se houver um arquivo nativo correspondente
- `dot_clean -s caminho/para/diretório` : segue os links simbólicos
- `dot_clean -v caminho/para/diretório` : imprime saída verbosa

## Links
- More information: https://keith.github.io/xcode-man-pages/dot_clean.1.html
