# ed

O editor de texto original do Unix.

## Usage
- `ed` : inicia uma sessão do editor interativo com um documento vazio
- `ed -p '> '` : inicia uma sessão do editor interativo com um documento vazio e um prompt específico
- `ed -v` : inicia uma sessão do editor interativo com erros compreensíveis para usuários
- `ed -q` : inicia uma sessão do editor interativo com um documento vazio e sem diagnósticos, contagens de bytes e prompt "!"
- `ed -l` : inicia uma sessão do editor interativo sem mudança no status de saída quando o comando falha
- `ed caminho/para/arquivo` : edita um arquivo específico (mostra a contagem de bytes do arquivo carregado)
- `,s/regex/substituto/g` : [Interativo] Substitui a string com um substituto específico em todas as linhas

## Links
- More information: https://www.gnu.org/software/ed/manual/ed_manual.html
