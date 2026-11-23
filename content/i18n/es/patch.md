# patch

Emparcha un archivo (o archivos) con un archivo diff. Ten en cuenta que los archivos diff deben ser generados por el comando `diff`.

## Usage
- `patch <parche.diff` : aplica un parche usando un archivo diff (los nombres de archivo deben incluirse en el archivo diff)
- `patch <parche.diff ruta/al/archivo` : aplica un parche a un archivo específico
- `patch <parche.diff ruta/al/archivo_de_entrada -o ruta/al/archivo_resultado` : emparcha un archivo escribiendo el resultado a un archivo diferente
- `patch <parche.diff -p 1` : aplica un parche al directorio actual
- `patch <parche.diff -R` : aplica el reverso de un parche

## Links
- More information: https://manned.org/patch
