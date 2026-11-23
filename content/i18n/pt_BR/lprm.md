# lprm

Cancela trabalhos de impressão na fila de um servidor.

## Usage
- `lprm` : cancela o trabalho atual na impressora padrão
- `lprm -h servidor:porta id_do_trabalho` : cancela um trabalho de um servidor específico
- `lprm -E id_do_trabalho1 id_do_trabalho2 ...` : cancela múltiplos trabalhos com uma conexão criptografada com o servidor
- `lprm -` : cancela todos os trabalhos
- `lprm -P destino/instância` : cancela o trabalho atual de uma impressora ou classe específica

## Links
- More information: https://openprinting.github.io/cups/doc/man-lprm.html
