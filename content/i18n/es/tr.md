# tr

Traduce caracteres: ejecuta sustituciones basadas en caracteres individuales y conjuntos de caracteres.

## Usage
- `tr <ruta/al/archivo find_character replace_character` : sustituye todas las apariciones de un carácter en un archivo e imprime el resultado
- `echo text | tr find_character replace_character` : sustituye todas las apariciones de un carácter de la salida de otro comando
- `tr <ruta/al/archivo 'abcd' 'jkmn'` : mapea cada carácter del primer conjunto al carácter correspondiente del segundo
- `tr <ruta/al/archivo -d 'input_characters'` : elimina todas las apariciones del conjunto de caracteres especificado de la entrada
- `tr <ruta/al/archivo -s 'input_characters'` : comprime una serie de caracteres idénticos a un solo carácter
- `tr <ruta/al/archivo "[:lower:]" "[:upper:]"` : traduce el contenido de un archivo a mayúsculas
- `tr <ruta/al/archivo -cd "[:print:]"` : elimina los caracteres no imprimibles de un archivo

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tr-invocation.html
