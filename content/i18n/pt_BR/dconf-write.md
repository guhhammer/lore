# dconf write

Escreve valores de chave nos bancos de dados dconf.

## Usage
- `dconf write /caminho/para/chave "valor"` : escreve um valor de chave específico
- `dconf write /caminho/para/chave "'string'"` : escreve uma string específica como valor de chave
- `dconf write /caminho/para/chave "5"` : escreve um inteiro específico como valor de chave
- `dconf write /caminho/para/chave "true|false"` : escreve um booleano específico como valor de chave
- `dconf write /caminho/para/chave "['primeiro', 'segundo', ...]"` : escreve um array específico como valor de chave
- `dconf write /caminho/para/chave "@as []"` : escreve um array vazio específico como valor de chave

## Links
- More information: https://manned.org/dconf
