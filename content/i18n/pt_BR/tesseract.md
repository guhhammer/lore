# tesseract

Motor de OCR (Reconhecimento Óptico de Caracteres).

## Usage
- `tesseract caminho/para/imagem.png saida` : reconhece o texto em uma imagem e salva-o no arquivo `saida.txt` (a extensão `.txt` é adicionada automaticamente)
- `tesseract -l deu caminho/para/imagem.png caminho/para/saida` : especifica uma linguagem personalizada (o padrão é inglês) com um código ISO 639-2 (ex. deu = Deutsch = Alemão)
- `tesseract --list-langs` : lista os códigos de idiomas da ISO 639-2 instalados
- `tesseract --psm 0..13 caminho/para/imagem.png caminho/para/saida` : especifica um modo de segmentação de página personalizado (o padrão é 3)
- `tesseract --help-psm` : lista os modos de segmentação de página e suas descrições

## Links
- More information: https://github.com/tesseract-ocr/tesseract/blob/main/doc/tesseract.1.asc
