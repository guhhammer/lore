# asciidoctor

Um processador que converte AsciiDoc em formatos publicáveis.

## Usage
- `asciidoctor caminho/para/arquivo.adoc` : converte um arquivo `.adoc` em HTML (formato padrão de saída)
- `asciidoctor -a stylesheet=caminho/para/estilos.css caminho/para/arquivo.adoc` : converte um arquivo `.adoc` em HTML e liga a uma folha de estilos CSS
- `asciidoctor -e caminho/para/arquivo.adoc` : converte um arquivo `.adoc` em um HTML embutível, removendo tudo exceto o corpo
- `asciidoctor -b pdf -r asciidoctor-pdf caminho/para/arquivo.adoc` : converte um arquivo `.adoc` em PDF usando a biblioteca `asciidoctor-pdf`

## Links
- More information: https://docs.asciidoctor.org/asciidoctor/latest/cli/man1/asciidoctor/
