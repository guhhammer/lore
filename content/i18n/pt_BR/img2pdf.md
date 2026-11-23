# img2pdf

Ferramenta de conversão sem perdas de imagens para PDF.

## Usage
- `img2pdf caminho/da/imagem1.ext caminho/da/imagem2.ext ... --output caminho/do/arquivo.pdf` : converte múltiplas imagens para um único PDF, cada imagem sendo uma página
- `img2pdf caminho/do/arquivo.gif --first-frame-only --output caminho/do/arquivo.pdf` : converte para PDF apenas o primeiro quadro de uma imagem com múltiplos quadros
- `img2pdf caminho/do/arquivo.ext --auto-orient --pagesize A4^T --border 2cm:5.1cm --output caminho/do/arquivo.pdf` : auto-orienta a imagem, usando uma página A4 em modo paisagem, e borda horizontal e vertical de tamanho específico
- `img2pdf caminho/do/arquivo.tiff --pagesize 30cmx20cm --imgsize 10cmx15cm --fit shrink --output caminho/do/arquivo.pdf` : encolhe apenas imagens maiores para um retângulo de dimensões específicas dentro de uma página de tamanho específico
- `img2pdf caminho/do/arquivo.png --title título --author autor --creationdate 1970-01-31 --keywords palavra_chave1 palavra_chave2 --subject assunto --output caminho/do/arquivo.pdf` : converte uma imagem para PDF e especifica os metadados do arquivo resultante

## Links
- More information: https://gitlab.mister-muffin.de/josch/img2pdf
