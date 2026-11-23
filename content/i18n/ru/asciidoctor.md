# asciidoctor

Преобразователь AsciiDoc файлов в другие форматы для публикации.

## Usage
- `asciidoctor путь/к/файлу.adoc` : преобразовать данный `.adoc` файл в HTML (выходной формат по умолчанию)
- `asciidoctor -a stylesheet=путь/к/таблице_стилей.css путь/к/файлу.adoc` : преобразовать данный `.adoc` файл в HTML и привязать таблицу стилей CSS
- `asciidoctor -e путь/к/файлу.adoc` : преобразовать данный `.adoc` файл во встраиваемый HTML, убрав всё, кроме самого текста
- `asciidoctor -b pdf -r asciidoctor-pdf путь/к/файлу.adoc` : преобразовать данный `.adoc` файл в PDF с помощью библиотеки `asciidoctor-pdf`

## Links
- More information: https://docs.asciidoctor.org/asciidoctor/latest/cli/man1/asciidoctor/
