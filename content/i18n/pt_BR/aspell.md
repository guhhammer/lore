# aspell

Verificador ortográfico interativo. Mais informações: <http://aspell.net/man-html/index.html>.

## Usage
- `aspell check caminho/para/arquivo` : verifica a ortografia do texto de um arquivo
- `cat caminho/para/arquivo | aspell list` : exibe as palavras escritas incorretamente no terminal
- `aspell dicts` : exibe os dicionários disponíveis
- `aspell --lang cs` : executa `aspell` utilizando uma língua diferente (informe o código ISO 639 da língua)
- `cat caminho/para/arquivo | aspell --personal lista_pessoal.pws list` : exibe os erros ortográficos no terminal e ignorando as palavras da lista pessoal
