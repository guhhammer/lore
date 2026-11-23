# asciidoctor

Un processeur qui convertit des fichiers AsciiDoc vers un format publiable.

## Usage
- `asciidoctor chemin/vers/fichier.adoc` : convertit un fichier `.adoc` vers un fichier HTML (le format de sortie par défaut)
- `asciidoctor -a stylesheet=chemin/vers/feuille_de_style.css chemin/vers/fichier.adoc` : convertit un fichier `.adoc` vers un fichier HTML et lie une feuille de style CSS
- `asciidoctor -e chemin/vers/fichier.adoc` : convertit un fichier `.adoc` vers un fichier HTML embarqué, en enlevant tout sauf le body
- `asciidoctor -b pdf -r asciidoctor-pdf chemin/vers/fichier.adoc` : convertit un fichier `.adoc` vers un PDF en utilisant la librairie `asciidoctor-pdf`

## Links
- More information: https://docs.asciidoctor.org/asciidoctor/latest/cli/man1/asciidoctor/
