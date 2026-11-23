# gsettings

Consulta e modifica configurações do dconf com validação de esquema.

## Usage
- `gsettings set org.exemplo.esquema chave_exemplo valor` : define o valor de uma chave. Falha se a chave não existe ou o valor está fora do intervalo
- `gsettings get org.exemplo.esquema chave_exemplo` : imprime o valor de uma chave ou o padrão fornecido pelo esquema se a chave não foi definida no `dconf`
- `gsettings reset org.exemplo.esquema chave_exemplo` : desfaz a definição de uma chave, para que o valor padrão do esquema seja usado
- `gsettings list-recursively` : exibe todos os esquemas, chaves e valores (não realocáveis)
- `gsettings list-recursively org.exemplo.esquema` : exibe todas as chaves e valores (padrão se não definido) de um esquema
- `gsettings range org.exemplo.esquema chave_exemplo` : exibe valores permitidos pelo esquema para uma chave (útil com chaves enumeráveis)
- `gsettings describe org.exemplo.esquema chave_exemplo` : exibe a descrição legível por humanos de uma chave

## Links
- More information: https://manned.org/gsettings
