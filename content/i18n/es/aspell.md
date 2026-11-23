# aspell

Corrector ortográfico interactivo. Más información: <http://aspell.net/man-html/index.html>.

## Usage
- `aspell check ruta/al/archivo` : revisa la ortografía de un solo archivo
- `cat ruta/al/archivo | aspell list` : lista las palabras mal escritas de `stdin`
- `aspell dicts` : muestra los idiomas disponibles en el diccionario
- `aspell --lang cs` : ejecuta `aspell` con un idioma diferente (toma el código de idioma ISO 639 de dos letras)
- `cat ruta/al/archivo | aspell --personal lista_personal_de_palabras.pws list` : lista las palabras mal escritas de `stdin` e ignora las palabras de la lista personal de palabras
