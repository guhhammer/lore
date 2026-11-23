# nano

Editor de texto de linha de comando. Um clone melhorado de `Pico`.

## Usage
- `nano` : inicia o editor
- `nano -I` : inicia o editor sem usar arquivos de configuração
- `nano caminho/para/arquivo1 caminho/para/arquivo2 ...` : abre arquivos específicos, passando para o próximo arquivos ao fechar o anterior
- `nano +linha,coluna caminho/para/arquivo` : abre um arquivo e posiciona o cursor na linha e coluna especificadas
- `nano -S caminho/para/arquivo` : abre um arquivo e habilita soft wrapping
- `nano -i caminho/para/arquivo` : abre um arquivo e indenta novas linhas de acordo com a indentação da linha anterior
- `nano -B caminho/para/arquivo` : abre um arquivo e cria um arquivo de backup (`caminho/para/arquivo~`) ao salvá-lo

## Links
- More information: https://nano-editor.org/dist/latest/nano.html
