# textutil

Manipula archivos de texto en varios formatos.

## Usage
- `textutil -info foo.rtf` : muestra información de `foo.rtf`
- `textutil -convert html foo.rtf` : convierte `foo.rtf` en `foo.html`
- `textutil foo.rtf -convert txt` : convierte texto enriquecido a texto normal
- `textutil -convert rtf -font Times -fontsize 10 foo.txt` : convierte `foo.txt` en `foo.rtf`, usando la fuente Times con un tamaño 10
- `textutil -cat html -title "Varios archivos" -output index.html *.rtf` : carga todos los archivos RTF en el directorio actual, concatena su contenido y escribe el resultado como `index.html` con el título HTML establecido en "Varios archivos"

## Links
- More information: https://keith.github.io/xcode-man-pages/textutil.1.html
