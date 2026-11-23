# flex

Generador de analizadores léxicos. Dada la especificación de un analizador léxico, genera código C para su implementación.

## Usage
- `flex analyzer.l` : genera un analizador a partir de un fichero Lex, almacenándolo en el archivo `lex.yy.c`
- `flex -t analyzer.l` : escribe el analizador en `stdout`
- `flex analyzer.l -o analyzer.c` : especifica el archivo de salida
- `flex -B analyzer.l` : genera un analizador por lotes en lugar de un analizador interactivo
- `cc ruta/a/lex.yy.c -o ejecutable` : compila un archivo C generado por Lex

## Links
- More information: https://manned.org/flex
