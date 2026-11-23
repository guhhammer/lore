# cancel

Cancela trabalhos de impressão.

## Usage
- `cancel` : cancela o trabalho atual da impressora padrão (definida com `lpoptions -d impressora`)
- `cancel -u nome_do_usuário` : cancela todos os trabalhos da impressora padrão que pertencem a um usuário específico
- `cancel impressora` : cancela o trabalho atual de uma impressora específica
- `cancel impressora-id_do_trabalho` : cancela um trabalho específico de uma impressora específica
- `cancel -a` : cancela todos os trabalhos de todas as impressoras
- `cancel -a impressora` : cancela todos os trabalhos de uma impressora específica
- `cancel -h servidor -x` : cancela o trabalho atual de um servidor específico e então deleta os arquivos de dados do trabalho

## Links
- More information: https://openprinting.github.io/cups/doc/man-cancel.html
