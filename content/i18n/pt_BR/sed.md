# sed

Edita texto de uma maneira programável.

## Usage
- `comando | sed 's/apple/mango/g'` : substitui todas as ocorrências de `apple` (`regex` básica) por `mango` (`regex` básica) em todas as linhas de entrada e imprime o resultado na `stdout`
- `comando | sed -E 's/(apple)/\U\1/g'` : substitui todas as ocorrências de `apple` (`regex` estendida) por `APPLE` (`regex` estendida) em todas as linhas de entrada e imprime o resultado na `stdout`
- `sed -i 's/apple/mango/g' caminho/para/arquivo` : substitui todas as ocorrências de `apple` (`regex` básica) por `mango` (`regex` básica) em um arquivo específico e sobrescreve o arquivo original
- `comando | sed -f caminho/para/script.sed` : executa um arquivo de script específico e imprime o resultado na `stdout`
- `comando | sed -n '1p'` : imprime apenas uma primeira linha na `stdout`
- `sed -i 1d caminho/para/arquivo` : exclui a primeira linha de um arquivo
- `sed -i '1i\sua nova linha de texto\' caminho/para/arquivo` : adiciona uma nova linha na primeira linha de um arquivo

## Links
- More information: https://www.gnu.org/software/sed/manual/sed.html
