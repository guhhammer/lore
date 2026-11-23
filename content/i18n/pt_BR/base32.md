# base32

Codifica ou decodifica um arquivo ou a entrada padrão (`stdin`) de/para Base32, para a saída padrão (`stdout`).

## Usage
- `base32 caminho/para/arquivo` : codifica um arquivo
- `base32 -w 0|76|... caminho/para/arquivo` : envolve a saída codificada em uma largura específica (`0` desabilita o encapsulamento)
- `base32 -d caminho/para/arquivo` : decodifica um arquivo
- `algum_comando | base32` : codifica a partir de `stdin`
- `algum_comando | base32 -d` : decodifica a partir de `stdin`

## Links
- More information: https://manned.org/base32
