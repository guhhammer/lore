# mv

Movimentação de arquivos entre diretórios, ou renomeá-los.

## Usage
- `mv percorso/del/arquivo percorso/del/destino` : move um arquivo para um diretório arbitrário
- `mv percorso/del/arquivo_1 percorso/del/arquivo_2 ... percorso/del/destino` : move arquivos para outro diretório, mantendo os nomes dos arquivos
- `mv -f percorso/del/arquivo percorso/del/destino` : não requisita confirmação para sobrescrição de arquivos
- `mv -i percorso/del/arquivo percorso/del/destino` : requisita confirmação para sobrescrição de arquivos, independentemente das permissões de arquivo
- `mv -n percorso/del/arquivo percorso/del/destino` : não sobrescrita arquivos existentes no diretório de destino
- `mv -v percorso/del/arquivo percorso/del/destino` : move os arquivos em modo Verbose, mostrando os arquivos após sua movimentação

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/mv-invocation.html
