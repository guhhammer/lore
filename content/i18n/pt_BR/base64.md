# base64

Codifica ou decodifica um arquivo ou a entrada padrão (`stdin`) de/para Base64, para a saída padrão (`stdout`).

## Usage
- `base64 caminho/para/arquivo` : codifica um arquivo
- `base64 -w 0|76|... caminho/para/arquivo` : envolve a saída codificada em uma largura específica (`0` desabilita o encapsulamento)
- `base64 -d caminho/para/arquivo` : decodifica um arquivo
- `algum_comando | base64` : codifica a partir de `stdin`
- `algum_comando | base64 -d` : decodifica a partir de `stdin`

## Links
- More information: https://manned.org/base64
