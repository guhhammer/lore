# cp

Copia arquivos e diretórios.

## Usage
- `cp caminho/para/arquivo_origem.ext caminho/para/arquivo_destino.ext` : copia um arquivo para outra localização
- `cp caminho/para/arquivo_origem.ext caminho/para/diretório_pai_destino` : copia um arquivo em outro diretório, mantendo o nome do arquivo
- `cp -r caminho/para/diretório_origem caminho/para/diretório_destino` : copia recursivamente o conteúdo de um diretório para outra localização (se o destino existe, o diretório é copiado dentro dele)
- `cp -vr caminho/para/diretório_origem caminho/para/diretório_destino` : copia um diretório recursivamente, em modo verboso (mostra arquivos que estão sendo copiados)
- `cp -t caminho/para/diretório_destino caminho/para/arquivo1 caminho/para/arquivo2 ...` : copia múltiplos arquivos de uma só vez para um diretório
- `cp -i *.txt caminho/para/diretório_destino` : copia arquivos de texto para outra localização, em modo interativo (pergunta ao usuário antes de sobrescrever)
- `cp -L link caminho/para/diretório_destino` : segue links simbólicos antes de copiar
- `cp -t caminho/para/diretório_destino caminho/para/arquivo_ou_diretório1 caminho/para/arquivo_ou_diretório2 ...` : usa o primeiro argumento como o diretório destino (útil para `xargs ... | cp -t destination_directory`)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cp-invocation.html
